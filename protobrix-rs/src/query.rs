use crate::builders::*;
use crate::error::ProtobrixError;
use crate::proto::*;
use std::str::FromStr;

/// Metadata for a queryable table
#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub columns: Vec<AdvancedTableColumn>,
    pub table_filterable: bool,
    pub action_buttons: Vec<ActionButton>,
    pub table_name: String,
}

/// Sort specification for a column
#[derive(Debug, Clone)]
pub struct SortColumn<C> {
    pub column: C,
    pub direction: SortDirection,
}

impl<C> SortColumn<C> {
    pub fn new(column: C, direction: SortDirection) -> Self {
        Self { column, direction }
    }

    pub fn asc(column: C) -> Self {
        Self {
            column,
            direction: SortDirection::Asc,
        }
    }

    pub fn desc(column: C) -> Self {
        Self {
            column,
            direction: SortDirection::Desc,
        }
    }
}

/// Trait for queryable tables
pub trait TableQueryable {
    type Column: FromStr<Err = ProtobrixError> + Clone;
    type QueryBuilder: TableQueryBuilder<Column = Self::Column>;

    fn metadata(&self) -> TableMetadata;
    fn query_builder(&self) -> Self::QueryBuilder;
}

/// Trait for building and executing table queries
pub trait TableQueryBuilder {
    type Column: FromStr<Err = ProtobrixError> + Clone;

    /// Apply global search across searchable columns
    fn search(&mut self, search: &str) -> &mut Self;

    /// Set pagination offset
    fn offset(&mut self, offset: u32) -> &mut Self;

    /// Set pagination limit
    fn limit(&mut self, limit: u32) -> &mut Self;

    /// Search within a specific column
    fn search_column(&mut self, column: Self::Column, search: &str) -> &mut Self;

    /// Filter by exact value match
    fn filter_column(&mut self, column: Self::Column, value: TableCellValue) -> &mut Self;

    /// Filter by range (min/max)
    fn filter_range(&mut self, column: Self::Column, range: RangeFilter) -> &mut Self;

    /// Add sorting by column and direction
    fn sort(&mut self, sorts: &[SortColumn<Self::Column>]) -> &mut Self;

    /// Execute query and return rows with only specified columns
    fn execute(self, columns: &[Self::Column]) -> Result<Vec<AdvancedTableRow>, ProtobrixError>;
}

/// Extension trait providing convenience methods for loading table data
pub trait TableQueryableExt: TableQueryable {
    /// Load rows based on an AdvancedTableRequest
    fn load_rows(
        &self,
        request: &AdvancedTableRequest,
    ) -> Result<Vec<AdvancedTableRow>, ProtobrixError>;

    /// Load a complete MainElement based on an AdvancedTableRequest
    fn load_table(
        &self,
        request: &AdvancedTableRequest,
        title: &str,
        url: &str,
    ) -> Result<MainElement, ProtobrixError>;
}

/// Apply request column configuration to metadata columns
///
/// If the request contains column configuration, this function:
/// - Reorders columns according to the request order
/// - Updates visibility (column index: 0 for hidden, 1-based for visible)
/// - Applies sort direction from the request
fn apply_column_configuration(
    metadata_columns: Vec<AdvancedTableColumn>,
    request: &AdvancedTableRequest,
) -> Vec<AdvancedTableColumn> {
    if request.columns.is_empty() {
        return metadata_columns;
    }

    // Create a map of metadata columns for quick lookup
    let mut col_map: std::collections::HashMap<String, AdvancedTableColumn> = metadata_columns
        .into_iter()
        .map(|col| (col.id.clone(), col))
        .collect();

    // Build final column list in request order, applying visibility and sort settings
    request
        .columns
        .iter()
        .enumerate()
        .filter_map(|(idx, req_col)| {
            col_map.get_mut(&req_col.id).map(|col| {
                // Update visibility (column index)
                if !req_col.hidden {
                    col.column = (idx + 1) as u32; // 1-based column index for visible columns
                } else {
                    col.column = 0; // Hidden
                }

                // Update sort direction
                if req_col.sort_index > 0 {
                    col.sort_direction = req_col.sort_direction;
                } else {
                    col.sort_direction = SortDirection::Unspecified as i32;
                }

                col.clone()
            })
        })
        .collect()
}

/// Blanket implementation of TableQueryableExt for all TableQueryable types
impl<T: TableQueryable> TableQueryableExt for T {
    fn load_rows(
        &self,
        request: &AdvancedTableRequest,
    ) -> Result<Vec<AdvancedTableRow>, ProtobrixError> {
        let mut builder = self.query_builder();

        // Apply global search
        if !request.search.is_empty() {
            builder.search(&request.search);
        }

        // Apply pagination
        builder.offset(request.offset);
        if request.limit > 0 {
            builder.limit(request.limit);
        }

        // Collect sorting columns (sort by sort_index)
        let mut sorts: Vec<(u32, &AdvancedTableRequestColumn)> = request
            .columns
            .iter()
            .filter(|col| col.sort_index > 0)
            .map(|col| (col.sort_index, col))
            .collect();
        sorts.sort_by_key(|(index, _)| *index);

        // Apply column-specific operations
        for col_request in &request.columns {
            // Parse column ID
            let column = T::Column::from_str(&col_request.id)?;

            // Apply column search
            if !col_request.search.is_empty() {
                builder.search_column(column.clone(), &col_request.search);
            }

            // Apply column filters (multiple values)
            for filter in &col_request.filters {
                builder.filter_column(column.clone(), filter.clone());
            }
        }

        // Apply sorting
        let sort_columns: Vec<SortColumn<T::Column>> = sorts
            .iter()
            .filter_map(|(_, col_request)| {
                let column = T::Column::from_str(&col_request.id).ok()?;
                let direction = SortDirection::try_from(col_request.sort_direction)
                    .unwrap_or(SortDirection::Unspecified);

                if direction != SortDirection::Unspecified {
                    Some(SortColumn::new(column, direction))
                } else {
                    None
                }
            })
            .collect();

        if !sort_columns.is_empty() {
            builder.sort(&sort_columns);
        }

        // Extract columns to return from request (excluding hidden columns)
        // If no columns specified in request, use all columns from metadata
        let columns_to_return: Vec<T::Column> = if request.columns.is_empty() {
            // Default to visible columns from metadata, sorted by column index
            let metadata = self.metadata();
            let mut visible_cols: Vec<_> = metadata
                .columns
                .iter()
                .filter(|col| col.column > 0) // Only visible columns
                .collect();

            // Sort by column index
            visible_cols.sort_by_key(|col| col.column);

            visible_cols
                .into_iter()
                .map(|col| T::Column::from_str(&col.id))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            request
                .columns
                .iter()
                .filter(|col| !col.hidden)
                .map(|col| T::Column::from_str(&col.id))
                .collect::<Result<Vec<_>, _>>()?
        };

        // Execute query
        builder.execute(&columns_to_return)
    }

    fn load_table(
        &self,
        request: &AdvancedTableRequest,
        title: &str,
        url: &str,
    ) -> Result<MainElement, ProtobrixError> {
        // Load rows
        let rows = self.load_rows(request)?;

        // Get metadata
        let metadata = self.metadata();

        // Build MainElement
        let mut builder = AdvancedTableBuilder::new()
            .title(title)
            .url(url)
            .filterable(metadata.table_filterable)
            .name(&metadata.table_name);

        // Apply request column configuration to metadata columns if present
        let final_columns = apply_column_configuration(metadata.columns, request);

        // Add columns
        for column in final_columns {
            builder = builder.add_column(column);
        }

        // Add action buttons
        for action_button in metadata.action_buttons {
            builder = builder.add_action_button(action_button);
        }

        // Add rows
        for row in rows {
            builder = builder.add_row(row);
        }

        Ok(builder.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock column enum for testing
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum TestColumn {
        Id,
        Name,
        Age,
    }

    impl ToString for TestColumn {
        fn to_string(&self) -> String {
            match self {
                TestColumn::Id => "id".to_string(),
                TestColumn::Name => "name".to_string(),
                TestColumn::Age => "age".to_string(),
            }
        }
    }

    impl FromStr for TestColumn {
        type Err = ProtobrixError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "id" => Ok(TestColumn::Id),
                "name" => Ok(TestColumn::Name),
                "age" => Ok(TestColumn::Age),
                _ => Err(ProtobrixError::Builder(format!("Invalid column: {}", s))),
            }
        }
    }

    // Mock query builder for testing
    #[derive(Debug, Clone)]
    struct TestQueryBuilder {
        search_query: String,
        offset: u32,
        limit: u32,
        column_searches: Vec<(TestColumn, String)>,
        column_filters: Vec<(TestColumn, TableCellValue)>,
        sorts: Vec<SortColumn<TestColumn>>,
    }

    impl TestQueryBuilder {
        fn new() -> Self {
            Self {
                search_query: String::new(),
                offset: 0,
                limit: 10,
                column_searches: Vec::new(),
                column_filters: Vec::new(),
                sorts: Vec::new(),
            }
        }
    }

    impl TableQueryBuilder for TestQueryBuilder {
        type Column = TestColumn;

        fn search(&mut self, search: &str) -> &mut Self {
            self.search_query = search.to_string();
            self
        }

        fn offset(&mut self, offset: u32) -> &mut Self {
            self.offset = offset;
            self
        }

        fn limit(&mut self, limit: u32) -> &mut Self {
            self.limit = limit;
            self
        }

        fn search_column(&mut self, column: Self::Column, search: &str) -> &mut Self {
            self.column_searches.push((column, search.to_string()));
            self
        }

        fn filter_column(&mut self, column: Self::Column, value: TableCellValue) -> &mut Self {
            self.column_filters.push((column, value));
            self
        }

        fn filter_range(&mut self, _column: Self::Column, _range: RangeFilter) -> &mut Self {
            self
        }

        fn sort(&mut self, sorts: &[SortColumn<Self::Column>]) -> &mut Self {
            self.sorts.extend_from_slice(sorts);
            self
        }

        fn execute(
            self,
            columns: &[Self::Column],
        ) -> Result<Vec<AdvancedTableRow>, ProtobrixError> {
            // Return mock data
            let mut rows = vec![
                AdvancedTableRow {
                    cells: vec![
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(1)),
                        },
                        TableCellValue {
                            value: Some(table_cell_value::Value::StringValue("Alice".to_string())),
                        },
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(30)),
                        },
                    ],
                    action_buttons: Vec::new(),
                    row_action: None,
                },
                AdvancedTableRow {
                    cells: vec![
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(2)),
                        },
                        TableCellValue {
                            value: Some(table_cell_value::Value::StringValue("Bob".to_string())),
                        },
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(25)),
                        },
                    ],
                    action_buttons: Vec::new(),
                    row_action: None,
                },
            ];

            // Filter rows based on column filters
            // Group filters by column for OR logic within same column
            use std::collections::HashMap;
            let mut filters_by_column: HashMap<TestColumn, Vec<&TableCellValue>> = HashMap::new();
            for (column, filter_value) in &self.column_filters {
                filters_by_column
                    .entry(column.clone())
                    .or_insert_with(Vec::new)
                    .push(filter_value);
            }

            // Apply filters with OR logic for same column, AND logic across columns
            for (column, filter_values) in &filters_by_column {
                rows.retain(|row| {
                    let col_idx = match column {
                        TestColumn::Id => 0,
                        TestColumn::Name => 1,
                        TestColumn::Age => 2,
                    };
                    if let Some(cell) = row.cells.get(col_idx) {
                        // OR logic: match any of the filter values for this column
                        filter_values
                            .iter()
                            .any(|filter_value| cell.value == filter_value.value)
                    } else {
                        false
                    }
                });
            }

            // Apply pagination
            let start = self.offset as usize;
            let end = (start + self.limit as usize).min(rows.len());
            rows = rows[start..end].to_vec();

            // Filter cells to only include requested columns
            for row in &mut rows {
                let mut new_cells = Vec::new();
                for col in columns {
                    let idx = match col {
                        TestColumn::Id => 0,
                        TestColumn::Name => 1,
                        TestColumn::Age => 2,
                    };
                    if let Some(cell) = row.cells.get(idx) {
                        new_cells.push(cell.clone());
                    }
                }
                row.cells = new_cells;
            }

            Ok(rows)
        }
    }

    // Mock table for testing
    struct TestTable;

    impl TableQueryable for TestTable {
        type Column = TestColumn;
        type QueryBuilder = TestQueryBuilder;

        fn metadata(&self) -> TableMetadata {
            TableMetadata {
                columns: vec![
                    AdvancedTableColumnBuilder::new("id", "ID")
                        .column_type(ColumnType::Int)
                        .sortable()
                        .column_index(1)
                        .build(),
                    AdvancedTableColumnBuilder::new("name", "Name")
                        .column_type(ColumnType::String)
                        .sortable()
                        .searchable()
                        .column_index(2)
                        .build(),
                    AdvancedTableColumnBuilder::new("age", "Age")
                        .column_type(ColumnType::Int)
                        .sortable()
                        .filterable()
                        .column_index(3)
                        .build(),
                ],
                table_filterable: true,
                action_buttons: Vec::new(),
                table_name: "test_table".to_string(),
            }
        }

        fn query_builder(&self) -> Self::QueryBuilder {
            TestQueryBuilder::new()
        }
    }

    #[test]
    fn test_load_rows_basic() {
        let table = TestTable;
        let request = AdvancedTableRequest {
            columns: vec![
                AdvancedTableRequestColumn {
                    id: "id".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "name".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "age".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
            ],
            search: String::new(),
            offset: 0,
            limit: 10,
        };

        let result = table.load_rows(&request);
        assert!(result.is_ok());
        let rows = result.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].cells.len(), 3);
    }

    #[test]
    fn test_load_rows_with_pagination() {
        let table = TestTable;
        let request = AdvancedTableRequest {
            columns: vec![
                AdvancedTableRequestColumn {
                    id: "id".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "name".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
            ],
            search: String::new(),
            offset: 1,
            limit: 1,
        };

        let result = table.load_rows(&request);
        assert!(result.is_ok());
        let rows = result.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].cells.len(), 2);
    }

    #[test]
    fn test_load_rows_with_filter() {
        let table = TestTable;
        let request = AdvancedTableRequest {
            columns: vec![AdvancedTableRequestColumn {
                id: "age".to_string(),
                search: String::new(),
                filters: vec![TableCellValue {
                    value: Some(table_cell_value::Value::IntValue(30)),
                }],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            }],
            search: String::new(),
            offset: 0,
            limit: 10,
        };

        let result = table.load_rows(&request);
        assert!(result.is_ok());
        let rows = result.unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_load_table() {
        let table = TestTable;
        let request = AdvancedTableRequest {
            columns: vec![
                AdvancedTableRequestColumn {
                    id: "id".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "name".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "age".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
            ],
            search: String::new(),
            offset: 0,
            limit: 10,
        };

        let result = table.load_table(&request, "Test Table", "/api/test");
        assert!(result.is_ok());
        let main_element = result.unwrap();
        assert_eq!(main_element.title, "Test Table");

        if let Some(main_element::Content::AdvancedTable(table)) = main_element.content {
            assert_eq!(table.url, "/api/test");
            assert_eq!(table.columns.len(), 3);
            assert_eq!(table.rows.len(), 2);
            assert!(table.table_filterable);
        } else {
            panic!("Expected AdvancedTable content");
        }
    }

    #[test]
    fn test_sort_column_constructors() {
        let sort_asc = SortColumn::asc(TestColumn::Name);
        assert_eq!(sort_asc.direction, SortDirection::Asc);

        let sort_desc = SortColumn::desc(TestColumn::Age);
        assert_eq!(sort_desc.direction, SortDirection::Desc);
    }

    #[test]
    fn test_hidden_columns() {
        let table = TestTable;
        let request = AdvancedTableRequest {
            columns: vec![
                AdvancedTableRequestColumn {
                    id: "id".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "name".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: false,
                },
                AdvancedTableRequestColumn {
                    id: "age".to_string(),
                    search: String::new(),
                    filters: Vec::new(),
                    sort_index: 0,
                    sort_direction: SortDirection::Unspecified as i32,
                    hidden: true, // This column should be excluded from results
                },
            ],
            search: String::new(),
            offset: 0,
            limit: 10,
        };

        let result = table.load_rows(&request);
        assert!(result.is_ok());
        let rows = result.unwrap();
        assert_eq!(rows.len(), 2);
        // Each row should only have 2 cells (id and name), not 3
        assert_eq!(rows[0].cells.len(), 2);
        assert_eq!(rows[1].cells.len(), 2);
    }
}
