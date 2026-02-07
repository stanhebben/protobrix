use protobrix_rs::*;
use std::str::FromStr;

// Step 1: Define a column enum for type safety
#[derive(Debug, Clone, PartialEq)]
enum UserColumn {
    Id,
    Name,
    Email,
    Age,
}

impl ToString for UserColumn {
    fn to_string(&self) -> String {
        match self {
            UserColumn::Id => "id".to_string(),
            UserColumn::Name => "name".to_string(),
            UserColumn::Email => "email".to_string(),
            UserColumn::Age => "age".to_string(),
        }
    }
}

impl FromStr for UserColumn {
    type Err = ProtobrixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "id" => Ok(UserColumn::Id),
            "name" => Ok(UserColumn::Name),
            "email" => Ok(UserColumn::Email),
            "age" => Ok(UserColumn::Age),
            _ => Err(ProtobrixError::InvalidColumn(s.to_string())),
        }
    }
}

// Step 2: Create a data structure to hold our data
#[derive(Clone)]
struct UserData {
    id: i32,
    name: String,
    email: String,
    age: i32,
}

// Step 3: Implement the query builder
struct UserQueryBuilder {
    data: Vec<UserData>,
    search_query: String,
    offset: u32,
    limit: u32,
    column_searches: Vec<(UserColumn, String)>,
    column_filters: Vec<(UserColumn, TableCellValue)>,
    sorts: Vec<SortColumn<UserColumn>>,
}

impl UserQueryBuilder {
    fn new(data: Vec<UserData>) -> Self {
        Self {
            data,
            search_query: String::new(),
            offset: 0,
            limit: 100,
            column_searches: Vec::new(),
            column_filters: Vec::new(),
            sorts: Vec::new(),
        }
    }
}

impl TableQueryBuilder for UserQueryBuilder {
    type Column = UserColumn;
    type Connection = ();

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
        // Range filtering implementation would go here
        self
    }

    fn sort(&mut self, sorts: &[SortColumn<Self::Column>]) -> &mut Self {
        self.sorts.extend_from_slice(sorts);
        self
    }

    fn execute(
        self,
        _conn: &mut Self::Connection,
        columns: &[Self::Column],
    ) -> Result<Vec<AdvancedTableRow>, ProtobrixError> {
        let mut filtered_data: Vec<&UserData> = self.data.iter().collect();

        // Apply global search
        if !self.search_query.is_empty() {
            filtered_data.retain(|user| {
                user.name
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
                    || user
                        .email
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
            });
        }

        // Apply column searches
        for (column, search) in &self.column_searches {
            let search_lower = search.to_lowercase();
            filtered_data.retain(|user| match column {
                UserColumn::Name => user.name.to_lowercase().contains(&search_lower),
                UserColumn::Email => user.email.to_lowercase().contains(&search_lower),
                _ => true,
            });
        }

        // Apply column filters
        for (column, filter_value) in &self.column_filters {
            filtered_data.retain(|user| match column {
                UserColumn::Id => {
                    if let Some(table_cell_value::Value::IntValue(val)) = &filter_value.value {
                        user.id == *val
                    } else {
                        false
                    }
                }
                UserColumn::Age => {
                    if let Some(table_cell_value::Value::IntValue(val)) = &filter_value.value {
                        user.age == *val
                    } else {
                        false
                    }
                }
                _ => true,
            });
        }

        // Apply sorting
        if !self.sorts.is_empty() {
            let mut data_vec: Vec<&UserData> = filtered_data;
            for sort in &self.sorts {
                data_vec.sort_by(|a, b| {
                    let cmp = match &sort.column {
                        UserColumn::Id => a.id.cmp(&b.id),
                        UserColumn::Name => a.name.cmp(&b.name),
                        UserColumn::Email => a.email.cmp(&b.email),
                        UserColumn::Age => a.age.cmp(&b.age),
                    };
                    match sort.direction {
                        SortDirection::Desc => cmp.reverse(),
                        _ => cmp,
                    }
                });
            }
            filtered_data = data_vec;
        }

        // Apply pagination
        let start = self.offset as usize;
        let end = (start + self.limit as usize).min(filtered_data.len());
        let paginated_data = &filtered_data[start..end];

        // Build rows with only requested columns
        let rows: Vec<AdvancedTableRow> = paginated_data
            .iter()
            .map(|user| {
                let mut cells = Vec::new();
                for col in columns {
                    let cell = match col {
                        UserColumn::Id => TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(user.id)),
                        },
                        UserColumn::Name => TableCellValue {
                            value: Some(table_cell_value::Value::StringValue(user.name.clone())),
                        },
                        UserColumn::Email => TableCellValue {
                            value: Some(table_cell_value::Value::StringValue(user.email.clone())),
                        },
                        UserColumn::Age => TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(user.age)),
                        },
                    };
                    cells.push(cell);
                }

                AdvancedTableRow {
                    cells,
                    action_buttons: vec![
                        ActionButtonBuilder::open_modal("Edit", &format!("/edit-user/{}", user.id))
                            .icon("fas:edit")
                            .build(),
                    ],
                    row_action: None,
                }
            })
            .collect();

        Ok(rows)
    }
}

// Step 4: Implement TableQueryable for our table
struct UserTable {
    users: Vec<UserData>,
}

impl UserTable {
    fn new() -> Self {
        // Sample data
        Self {
            users: vec![
                UserData {
                    id: 1,
                    name: "Alice Johnson".to_string(),
                    email: "alice@example.com".to_string(),
                    age: 30,
                },
                UserData {
                    id: 2,
                    name: "Bob Smith".to_string(),
                    email: "bob@example.com".to_string(),
                    age: 25,
                },
                UserData {
                    id: 3,
                    name: "Charlie Brown".to_string(),
                    email: "charlie@example.com".to_string(),
                    age: 35,
                },
                UserData {
                    id: 4,
                    name: "Diana Prince".to_string(),
                    email: "diana@example.com".to_string(),
                    age: 28,
                },
                UserData {
                    id: 5,
                    name: "Eve Adams".to_string(),
                    email: "eve@example.com".to_string(),
                    age: 32,
                },
            ],
        }
    }
}

impl TableQueryable for UserTable {
    type Column = UserColumn;
    type QueryBuilder = UserQueryBuilder;
    type Connection = ();

    fn metadata(&self, _conn: &mut Self::Connection) -> TableMetadata {
        TableMetadata {
            table_name: "users".to_string(),
            table_filterable: true,
            action_buttons: vec![],
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
                AdvancedTableColumnBuilder::new("email", "Email")
                    .column_type(ColumnType::String)
                    .searchable()
                    .column_index(3)
                    .build(),
                AdvancedTableColumnBuilder::new("age", "Age")
                    .column_type(ColumnType::Int)
                    .sortable()
                    .filterable()
                    .range_filterable(
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(0)),
                        },
                        TableCellValue {
                            value: Some(table_cell_value::Value::IntValue(100)),
                        },
                    )
                    .column_index(4)
                    .build(),
            ],
        }
    }

    fn query_builder(&self) -> Self::QueryBuilder {
        UserQueryBuilder::new(self.users.clone())
    }
}

fn main() {
    println!("=== Queryable Table Example ===\n");

    let table = UserTable::new();

    // Example 1: Load all users
    println!("Example 1: Load all users");
    let request = AdvancedTableRequest {
        columns: vec![
            AdvancedTableRequestColumn {
                id: "id".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
            AdvancedTableRequestColumn {
                id: "name".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
            AdvancedTableRequestColumn {
                id: "email".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
            AdvancedTableRequestColumn {
                id: "age".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
        ],
        search: String::new(),
        offset: 0,
        limit: 10,
    };

    match table.load_table(
        &mut (),
        request,
        "All Users".to_string(),
        "/api/users".to_string(),
    ) {
        Ok(main_element) => {
            println!("✓ Loaded table: {}", main_element.title);
            if let Some(main_element::Content::AdvancedTable(t)) = main_element.content {
                println!("  - Columns: {}", t.columns.len());
                println!("  - Rows: {}", t.rows.len());
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 2: Search for "Alice"
    println!("\nExample 2: Search for 'Alice'");
    let request = AdvancedTableRequest {
        columns: vec![
            AdvancedTableRequestColumn {
                id: "id".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
            AdvancedTableRequestColumn {
                id: "name".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
        ],
        search: "Alice".to_string(),
        offset: 0,
        limit: 10,
    };

    match table.load_rows(&mut (), request) {
        Ok(rows) => {
            println!("✓ Found {} row(s)", rows.len());
            for row in rows {
                if let Some(table_cell_value::Value::StringValue(name)) =
                    row.cells.get(1).and_then(|c| c.value.as_ref())
                {
                    println!("  - {}", name);
                }
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 3: Filter by age = 30
    println!("\nExample 3: Filter by age = 30");
    let request = AdvancedTableRequest {
        columns: vec![
            AdvancedTableRequestColumn {
                id: "name".to_string(),
                search: String::new(),
                filters: vec![],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
            AdvancedTableRequestColumn {
                id: "age".to_string(),
                search: String::new(),
                filters: vec![TableCellValue {
                    value: Some(table_cell_value::Value::IntValue(30)),
                }],
                sort_index: 0,
                sort_direction: SortDirection::Unspecified as i32,
                hidden: false,
            },
        ],
        search: String::new(),
        offset: 0,
        limit: 10,
    };

    match table.load_rows(&mut (), request) {
        Ok(rows) => {
            println!("✓ Found {} row(s)", rows.len());
            for row in rows {
                if let (
                    Some(table_cell_value::Value::StringValue(name)),
                    Some(table_cell_value::Value::IntValue(age)),
                ) = (
                    row.cells.get(0).and_then(|c| c.value.as_ref()),
                    row.cells.get(1).and_then(|c| c.value.as_ref()),
                ) {
                    println!("  - {} (age: {})", name, age);
                }
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 4: Sort by name ascending
    println!("\nExample 4: Sort by name (ascending)");
    let request = AdvancedTableRequest {
        columns: vec![AdvancedTableRequestColumn {
            id: "name".to_string(),
            search: String::new(),
            filters: vec![],
            sort_index: 1,
            sort_direction: SortDirection::Asc as i32,
            hidden: false,
        }],
        search: String::new(),
        offset: 0,
        limit: 10,
    };

    match table.load_rows(&mut (), request) {
        Ok(rows) => {
            println!("✓ Found {} row(s) (sorted)", rows.len());
            for row in rows {
                if let Some(table_cell_value::Value::StringValue(name)) =
                    row.cells.get(0).and_then(|c| c.value.as_ref())
                {
                    println!("  - {}", name);
                }
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 5: Pagination
    println!("\nExample 5: Pagination (offset=2, limit=2)");
    let request = AdvancedTableRequest {
        columns: vec![AdvancedTableRequestColumn {
            id: "name".to_string(),
            search: String::new(),
            filters: vec![],
            sort_index: 0,
            sort_direction: SortDirection::Unspecified as i32,
            hidden: false,
        }],
        search: String::new(),
        offset: 2,
        limit: 2,
    };

    match table.load_rows(&mut (), request) {
        Ok(rows) => {
            println!("✓ Found {} row(s)", rows.len());
            for row in rows {
                if let Some(table_cell_value::Value::StringValue(name)) =
                    row.cells.get(0).and_then(|c| c.value.as_ref())
                {
                    println!("  - {}", name);
                }
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n=== All examples completed successfully! ===");
}
