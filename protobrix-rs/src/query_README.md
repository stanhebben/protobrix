# Query Module

The query module provides a trait-based system for creating queryable tables that work seamlessly with `AdvancedTableRequest`.

## Overview

The query system consists of three main traits:

1. **`TableQueryable`** - Defines a queryable table with metadata and a query builder
2. **`TableQueryBuilder`** - Defines how to build and execute queries
3. **`TableQueryableExt`** - Extension trait providing convenience methods (automatically implemented)

## Key Features

- **Type-safe column references** using associated types
- **Flexible query building** with method chaining
- **Automatic request handling** from `AdvancedTableRequest`
- **Column selection** at query execution time
- **Implementation-agnostic** - works with any data source

## Quick Start

### 1. Define Your Column Enum

```rust
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
```

### 2. Implement TableQueryBuilder

```rust
struct UserQueryBuilder {
    // Your query state (database connection, filters, etc.)
}

impl TableQueryBuilder for UserQueryBuilder {
    type Column = UserColumn;
    
    fn search(&mut self, search: &str) -> &mut Self {
        // Apply global search logic
        self
    }
    
    fn search_column(&mut self, column: Self::Column, search: &str) -> &mut Self {
        // Apply column-specific search
        self
    }
    
    fn filter_column(&mut self, column: Self::Column, value: TableCellValue) -> &mut Self {
        // Apply exact filter
        self
    }
    
    fn filter_range(&mut self, column: Self::Column, range: RangeFilter) -> &mut Self {
        // Apply range filter
        self
    }
    
    fn sort(&mut self, sorts: &[SortColumn<Self::Column>]) -> &mut Self {
        // Add sorting (can handle multiple sort columns)
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
    
    fn execute(&self, columns: &[Self::Column]) -> Result<Vec<AdvancedTableRow>, ProtobrixError> {
        // Execute query and return rows with only specified columns
        Ok(vec![/* rows */])
    }
}
```

### 3. Implement TableQueryable

```rust
struct UserTable {
    // Your data source (database connection, etc.)
}

impl TableQueryable for UserTable {
    type Column = UserColumn;
    type QueryBuilder = UserQueryBuilder;
    
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
                // ... more columns
            ],
            table_filterable: true,
        }
    }
    
    fn query_builder(&self) -> Self::QueryBuilder {
        UserQueryBuilder::new(/* ... */)
    }
}
```

### 4. Use in Your Endpoints

```rust
// With actix-web
async fn users_endpoint(request: AdvancedTableRequest) -> MainElement {
    let table = UserTable::new();
    table.load_table(&request, "Users", "/api/users/data").unwrap()
}

// Or just get the rows
async fn users_data(request: AdvancedTableRequest) -> Vec<AdvancedTableRow> {
    let table = UserTable::new();
    table.load_rows(&request).unwrap()
}
```

## API Reference

### TableMetadata

Holds table metadata:

```rust
pub struct TableMetadata {
    pub columns: Vec<AdvancedTableColumn>,
    pub table_filterable: bool,
}
```

### SortColumn

Specifies sorting for a column:

```rust
pub struct SortColumn<C> {
    pub column: C,
    pub direction: SortDirection,
}

// Convenience constructors
SortColumn::asc(column)
SortColumn::desc(column)
SortColumn::new(column, direction)
```

### TableQueryable Trait

```rust
pub trait TableQueryable {
    type Column: ToString + FromStr<Err = ProtobrixError> + Clone;
    type QueryBuilder: TableQueryBuilder<Column = Self::Column>;
    
    fn metadata(&self) -> TableMetadata;
    fn query_builder(&self) -> Self::QueryBuilder;
}
```

### TableQueryBuilder Trait

```rust
pub trait TableQueryBuilder {
    type Column: ToString + FromStr<Err = ProtobrixError> + Clone;
    
    fn search(&mut self, search: &str) -> &mut Self;
    fn offset(&mut self, offset: u32) -> &mut Self;
    fn limit(&mut self, limit: u32) -> &mut Self;
    fn search_column(&mut self, column: Self::Column, search: &str) -> &mut Self;
    fn filter_column(&mut self, column: Self::Column, value: TableCellValue) -> &mut Self;
    fn filter_range(&mut self, column: Self::Column, range: RangeFilter) -> &mut Self;
    fn sort(&mut self, sorts: &[SortColumn<Self::Column>]) -> &mut Self;
    fn execute(&self, columns: &[Self::Column]) -> Result<Vec<AdvancedTableRow>, ProtobrixError>;
}
```

### TableQueryableExt Trait

Automatically implemented for all `TableQueryable` types:

```rust
pub trait TableQueryableExt: TableQueryable {
    fn load_rows(&self, request: &AdvancedTableRequest) 
        -> Result<Vec<AdvancedTableRow>, ProtobrixError>;
    
    fn load_table(&self, request: &AdvancedTableRequest, title: &str, url: &str) 
        -> Result<MainElement, ProtobrixError>;
}
```

## Examples

See `examples/queryable_table.rs` for a complete working example with:
- In-memory data source
- Global search
- Column-specific search
- Filtering
- Sorting
- Pagination

## Design Rationale

### Why Associated Types for Columns?

Using associated types for columns provides:

1. **Type Safety**: Column names are validated at compile time
2. **IDE Support**: Autocomplete and refactoring work correctly
3. **Clear API**: `UserColumn::Name` is clearer than `"name"`
4. **Easy Conversion**: Simple `ToString`/`FromStr` for protobuf layer

### Why Separate Traits?

- **`TableQueryable`**: Defines what the table is (metadata + builder factory)
- **`TableQueryBuilder`**: Defines how to query (implementation-specific)
- **`TableQueryableExt`**: Provides high-level convenience (works with any implementation)

This separation allows:
- Different data sources (SQL, NoSQL, in-memory)
- Reusable query logic
- Easy testing with mock implementations

## Testing

The module includes comprehensive tests with a mock implementation. Run:

```bash
cargo test query
```

## Integration with Actix-web

The query system integrates seamlessly with the actix-web extractor:

```rust
use actix_web::{web, App, HttpServer};
use protobrix_rs::*;

async fn users_data(request: AdvancedTableRequest) -> MainElement {
    let table = UserTable::new();
    table.load_table(&request, "Users", "/api/users/data")
        .unwrap_or_else(|e| {
            // Handle error and return error page
            StaticPageBuilder::new()
                .title("Error")
                .add_paragraph(
                    ParagraphBuilder::new()
                        .add_text(&format!("Error: {}", e))
                        .build()
                )
                .build()
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/users/data", web::post().to(users_data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
