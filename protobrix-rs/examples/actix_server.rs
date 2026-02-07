use actix_web::{App, HttpServer, web};
use protobrix_rs::*;

/// Static page endpoint
async fn static_page() -> MainElement {
    StaticPageBuilder::new()
        .title("Welcome Page")
        .add_action_button(
            ActionButtonBuilder::go_to_url("Go Home", "/")
                .icon("fas:home")
                .build(),
        )
        .add_paragraph(
            ParagraphBuilder::new()
                .add_text("Welcome to Protobrix! ")
                .add_span(
                    TextSpanBuilder::new("This is a demonstration")
                        .bold()
                        .build(),
                )
                .add_text(" of the static page builder.")
                .build(),
        )
        .add_list(
            ListBuilder::new()
                .add_item(0, "First item")
                .add_item(0, "Second item")
                .add_item(1, "Nested item")
                .add_item(0, "Third item")
                .build(),
        )
        .add_code_block(
            CodeBlockBuilder::new("rust")
                .add_text("fn main() {\n    println!(\"Hello, world!\");\n}")
                .build(),
        )
        .build()
}

/// Simple table endpoint
async fn simple_table() -> MainElement {
    SimpleTableBuilder::new()
        .title("User List")
        .add_action_button(
            ActionButtonBuilder::open_modal("Add User", "/add-user")
                .icon("fas:plus")
                .build(),
        )
        .add_header_row(
            SimpleTableRowBuilder::header()
                .add_cell("ID")
                .add_cell("Name")
                .add_cell("Email")
                .build(),
        )
        .add_data_row(
            SimpleTableRowBuilder::data()
                .add_cell("1")
                .add_cell("John Doe")
                .add_cell("john@example.com")
                .build(),
        )
        .add_data_row(
            SimpleTableRowBuilder::data()
                .add_cell("2")
                .add_cell("Jane Smith")
                .add_cell("jane@example.com")
                .build(),
        )
        .build()
}

/// Advanced table endpoint
async fn advanced_table() -> MainElement {
    AdvancedTableBuilder::new()
        .title("Advanced User Table")
        .url("/api/users/data")
        .add_action_button(
            ActionButtonBuilder::go_to_url("Export", "/api/users/export")
                .icon("fas:download")
                .build(),
        )
        .add_column(
            AdvancedTableColumnBuilder::new("id", "ID")
                .column_type(ColumnType::Int)
                .sortable()
                .column_index(0)
                .build(),
        )
        .add_column(
            AdvancedTableColumnBuilder::new("name", "Name")
                .column_type(ColumnType::String)
                .sortable()
                .searchable()
                .column_index(1)
                .build(),
        )
        .add_column(
            AdvancedTableColumnBuilder::new("email", "Email")
                .column_type(ColumnType::String)
                .searchable()
                .column_index(2)
                .build(),
        )
        .add_column(
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
                .column_index(3)
                .build(),
        )
        .filterable(true)
        .add_row(
            AdvancedTableRowBuilder::new()
                .add_int_cell(1)
                .add_string_cell("John Doe")
                .add_string_cell("john@example.com")
                .add_int_cell(30)
                .add_action_button(
                    ActionButtonBuilder::open_modal("Edit", "/edit-user/1")
                        .icon("fas:edit")
                        .build(),
                )
                .build(),
        )
        .add_row(
            AdvancedTableRowBuilder::new()
                .add_int_cell(2)
                .add_string_cell("Jane Smith")
                .add_string_cell("jane@example.com")
                .add_int_cell(25)
                .add_action_button(
                    ActionButtonBuilder::open_modal("Edit", "/edit-user/2")
                        .icon("fas:edit")
                        .build(),
                )
                .build(),
        )
        .build()
}

/// Advanced table data endpoint - handles AdvancedTableRequest
async fn advanced_table_data(request: AdvancedTableRequest) -> MainElement {
    // In a real application, you would use the request parameters to filter, sort, and paginate data
    println!("Received advanced table request:");
    println!("  Search: {}", request.search);
    println!("  Offset: {}", request.offset);
    println!("  Limit: {}", request.limit);
    println!("  Columns: {} filters/sorts", request.columns.len());

    // For demonstration, return a simple response
    AdvancedTableBuilder::new()
        .title("Filtered Users")
        .url("/api/users/data")
        .add_column(
            AdvancedTableColumnBuilder::new("id", "ID")
                .column_type(ColumnType::Int)
                .column_index(0)
                .build(),
        )
        .add_column(
            AdvancedTableColumnBuilder::new("name", "Name")
                .column_type(ColumnType::String)
                .column_index(1)
                .build(),
        )
        .add_row(
            AdvancedTableRowBuilder::new()
                .add_int_cell(1)
                .add_string_cell("Filtered Result")
                .build(),
        )
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Protobrix example server on http://127.0.0.1:8080");
    println!("\nAvailable endpoints:");
    println!("  GET  /static-page       - Static page example");
    println!("  GET  /simple-table      - Simple table example");
    println!("  GET  /advanced-table    - Advanced table example");
    println!("  POST /api/users/data    - Advanced table data endpoint");
    println!("\nTry with different Accept headers:");
    println!("  curl -H 'Accept: application/json' http://127.0.0.1:8080/static-page");
    println!("  curl -H 'Accept: application/x-protobuf' http://127.0.0.1:8080/static-page");

    HttpServer::new(|| {
        App::new()
            .route("/static-page", web::get().to(static_page))
            .route("/simple-table", web::get().to(simple_table))
            .route("/advanced-table", web::get().to(advanced_table))
            .route("/api/users/data", web::post().to(advanced_table_data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
