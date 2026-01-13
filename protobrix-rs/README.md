# Protobrix-RS

A Rust library for building UI definitions using Protocol Buffers and JSON, with seamless Actix-web integration.

## Features

- **Builder Pattern API**: Ergonomic builders for constructing UI elements
- **Queryable Tables**: Type-safe trait system for building queryable tables with filtering, sorting, and pagination
- **Content Negotiation**: Automatic serialization/deserialization based on HTTP headers
- **Actix-web Integration**: Built-in extractors and responders for Actix-web 4.4
- **Dual Format Support**: Both JSON and Protocol Buffers
- **Type Safety**: Strongly typed builders with compile-time guarantees

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
protobrix-rs = "0.1.0"
```

For Actix-web integration (enabled by default):

```toml
[dependencies]
protobrix-rs = { version = "0.1.0", features = ["actix"] }
```

## Quick Start

### Building a Static Page

```rust
use protobrix_rs::*;

let page = StaticPageBuilder::new()
    .title("Welcome")
    .add_paragraph(
        ParagraphBuilder::new()
            .add_text("Hello, ")
            .add_span(TextSpanBuilder::new("world").bold().build())
            .build()
    )
    .add_list(
        ListBuilder::new()
            .add_item(0, "First item")
            .add_item(0, "Second item")
            .build()
    )
    .build();
```

### Building a Simple Table

```rust
use protobrix_rs::*;

let table = SimpleTableBuilder::new()
    .title("Users")
    .add_header_row(
        SimpleTableRowBuilder::header()
            .add_cell("Name")
            .add_cell("Email")
            .build()
    )
    .add_data_row(
        SimpleTableRowBuilder::data()
            .add_cell("John Doe")
            .add_cell("john@example.com")
            .build()
    )
    .build();
```

### Building an Advanced Table

```rust
use protobrix_rs::*;

let table = AdvancedTableBuilder::new()
    .title("Advanced Users")
    .url("/api/data")  // Optional: defaults to current URL if not specified
    .add_column(
        AdvancedTableColumnBuilder::new("id", "ID")
            .column_type(ColumnType::ColumnTypeInt)
            .sortable()
            .build()
    )
    .add_column(
        AdvancedTableColumnBuilder::new("name", "Name")
            .column_type(ColumnType::ColumnTypeString)
            .sortable()
            .searchable()
            .build()
    )
    .add_row(
        AdvancedTableRowBuilder::new()
            .add_int_cell(1)
            .add_string_cell("John Doe")
            .build()
    )
    .filterable(true)
    .build();
```

### Building Queryable Tables

For tables that need to handle complex queries (filtering, sorting, pagination), use the query trait system:

```rust
use protobrix_rs::*;
use std::str::FromStr;

// 1. Define your column enum
#[derive(Debug, Clone, PartialEq)]
enum UserColumn {
    Id,
    Name,
    Email,
}

impl ToString for UserColumn {
    fn to_string(&self) -> String {
        match self {
            UserColumn::Id => "id".to_string(),
            UserColumn::Name => "name".to_string(),
            UserColumn::Email => "email".to_string(),
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
            _ => Err(ProtobrixError::InvalidColumn(s.to_string())),
        }
    }
}

// 2. Implement TableQueryable for your data source
struct UserTable { /* your data source */ }

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
                // ... more columns
            ],
            table_filterable: true,
        }
    }
    
    fn query_builder(&self) -> Self::QueryBuilder {
        UserQueryBuilder::new(/* ... */)
    }
}

// 3. Use in your endpoint
async fn users_endpoint(request: AdvancedTableRequest) -> MainElement {
    let table = UserTable::new();
    table.load_table(&request, "Users", "/api/users/data").unwrap()
}
```

See [`src/query_README.md`](src/query_README.md) for detailed documentation and [`examples/queryable_table.rs`](examples/queryable_table.rs) for a complete example.

## Actix-web Integration

### Automatic Response Serialization

The `MainElement` type implements `Responder`, automatically serializing based on the `Accept` header:

```rust
use actix_web::{web, App, HttpServer};
use protobrix_rs::*;

async fn get_page() -> MainElement {
    StaticPageBuilder::new()
        .title("My Page")
        .add_paragraph(ParagraphBuilder::new().add_text("Hello").build())
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/page", web::get().to(get_page))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

**Content Negotiation:**
- `Accept: application/json` → JSON response
- `Accept: application/x-protobuf` → Protobuf response
- `Accept: */*` or missing → JSON response (default)

### Automatic Request Deserialization

The `AdvancedTableRequest` type implements `FromRequest`, automatically deserializing based on the `Content-Type` header:

```rust
use actix_web::{web, App, HttpResponse};
use protobrix_rs::*;

async fn handle_table_request(request: AdvancedTableRequest) -> HttpResponse {
    // Use request.search, request.offset, request.limit, etc.
    println!("Search: {}", request.search);
    println!("Offset: {}, Limit: {}", request.offset, request.limit);
    
    HttpResponse::Ok().finish()
}

// In your app:
App::new()
    .route("/api/data", web::post().to(handle_table_request))
```

**Content Negotiation:**
- `Content-Type: application/json` → Deserialize from JSON
- `Content-Type: application/x-protobuf` → Deserialize from Protobuf
- Missing or unsupported → 406 Not Acceptable error

## Builder API Reference

### Main Builders

All three main builders directly produce `MainElement`:

#### StaticPageBuilder

```rust
StaticPageBuilder::new()
    .title("Page Title")                    // Main element title
    .page_title("Different Page Title")     // Optional: different title for the page content
    .add_action_button(button)              // Add action buttons
    .add_paragraph(paragraph)               // Add paragraph element
    .add_list(list)                         // Add list element
    .add_simple_table(table)                // Add simple table element
    .add_code_block(code_block)             // Add code block element
    .build()                                // Returns MainElement
```

#### SimpleTableBuilder

```rust
SimpleTableBuilder::new()
    .title("Table Title")                   // Main element title
    .add_action_button(button)              // Add action buttons
    .add_header_row(row)                    // Add header row
    .add_data_row(row)                      // Add data row
    .add_row(row)                           // Add any row
    .build()                                // Returns MainElement
```

#### AdvancedTableBuilder

```rust
AdvancedTableBuilder::new()                 // Create new builder
    .title("Table Title")                   // Main element title
    .url("/api/endpoint")                   // Optional: data endpoint URL (defaults to current URL)
    .add_action_button(button)              // Add action buttons
    .add_column(column)                     // Add column definition
    .add_row(row)                           // Add data row
    .filterable(true)                       // Enable table filtering
    .build()                                // Returns MainElement
```

### Helper Builders

#### TextSpanBuilder

```rust
TextSpanBuilder::new("text")
    .color("red")
    .bold()
    .italic()
    .build()
```

#### ParagraphBuilder

```rust
ParagraphBuilder::new()
    .add_text("plain text")
    .add_span(text_span)
    .add_span_builder(text_span_builder)
    .build()
```

#### ListBuilder

```rust
ListBuilder::new()
    .add_item(0, "Level 0 item")
    .add_item(1, "Level 1 item (nested)")
    .add_item_with_spans(0, vec![span1, span2])
    .build()
```

#### ActionButtonBuilder

```rust
ActionButtonBuilder::new("Button Label")
    .icon("fas:check")
    .go_to_url("/url")           // Or: .open_page("/url") or .open_modal("/url")
    .build()?                     // Returns Result<ActionButton, ProtobrixError>
```

#### SimpleTableRowBuilder

```rust
SimpleTableRowBuilder::header()  // Or: ::data() or ::new(style)
    .add_cell("Cell text")
    .add_cell_with_spans(vec![span1, span2])
    .build()
```

#### AdvancedTableColumnBuilder

```rust
AdvancedTableColumnBuilder::new("column_id", "Column Title")
    .description("Column description")
    .column_type(ColumnType::ColumnTypeString)
    .sortable()
    .searchable()
    .filterable()
    .possible_values(vec![value1, value2])
    .decimal_digits(2)
    .unit("kg")
    .range_filterable(min_value, max_value)
    .column_index(0)
    .sort_direction(SortDirection::SortDirectionAsc)
    .build()
```

#### AdvancedTableRowBuilder

```rust
AdvancedTableRowBuilder::new()
    .add_string_cell("text")
    .add_int_cell(42)
    .add_double_cell(3.14)
    .add_float_cell(2.71)
    .add_bool_cell(true)
    .add_cell(custom_cell_value)
    .add_action_button(button)
    .build()
```

#### CodeBlockBuilder

```rust
CodeBlockBuilder::new("rust")
    .add_text("fn main() {}")
    .add_span(text_span)
    .build()
```

## Error Handling

The library uses `ProtobrixError` for all errors:

```rust
pub enum ProtobrixError {
    ProtobufDecode(prost::DecodeError),
    ProtobufEncode(prost::EncodeError),
    Json(serde_json::Error),
    UnsupportedContentType(String),
    MissingContentType,
    Payload(String),
    Builder(String),
}
```

When using Actix-web, errors are automatically converted to appropriate HTTP responses:
- `ProtobufDecode`, `Json`, `Payload` → 400 Bad Request
- `UnsupportedContentType`, `MissingContentType` → 406 Not Acceptable
- `ProtobufEncode`, `Builder` → 500 Internal Server Error

## Examples

Run the example server:

```bash
cargo run --example actix_server
```

Run the queryable table example:

```bash
cargo run --example queryable_table
```

Then test with curl:

```bash
# Get JSON response
curl -H "Accept: application/json" http://127.0.0.1:8080/static-page

# Get Protobuf response
curl -H "Accept: application/x-protobuf" http://127.0.0.1:8080/static-page

# Post JSON request
curl -X POST -H "Content-Type: application/json" \
  -d '{"search":"test","offset":0,"limit":10,"columns":[]}' \
  http://127.0.0.1:8080/api/users/data

# Post Protobuf request
curl -X POST -H "Content-Type: application/x-protobuf" \
  --data-binary @request.bin \
  http://127.0.0.1:8080/api/users/data
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
