#[cfg(test)]
mod tests {
    use crate::builders::*;
    use crate::proto::*;

    #[test]
    fn test_text_span_builder() {
        let span = TextSpanBuilder::new("Hello")
            .bold()
            .italic()
            .color("red")
            .build();

        assert_eq!(span.text, "Hello");
        assert!(span.style.is_some());
        let style = span.style.unwrap();
        assert_eq!(style.color, "red");
        assert!(style.bold);
        assert!(style.italic);
    }

    #[test]
    fn test_paragraph_builder() {
        let paragraph = ParagraphBuilder::new()
            .add_text("Hello ")
            .add_span(TextSpanBuilder::new("world").bold().build())
            .build();

        assert_eq!(paragraph.text_spans.len(), 2);
        assert_eq!(paragraph.text_spans[0].text, "Hello ");
        assert_eq!(paragraph.text_spans[1].text, "world");
    }

    #[test]
    fn test_list_builder() {
        let list = ListBuilder::new()
            .add_item(0, "Item 1")
            .add_item(1, "Nested item")
            .add_item(0, "Item 2")
            .build();

        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0].level, 0);
        assert_eq!(list.items[1].level, 1);
        assert_eq!(list.items[2].level, 0);
    }

    #[test]
    fn test_action_button_builder() {
        let button = ActionButtonBuilder::new("Click me")
            .icon("fas:check")
            .go_to_url("/home")
            .build()
            .unwrap();

        assert_eq!(button.label, "Click me");
        assert_eq!(button.icon, "fas:check");
        assert!(button.action.is_some());
    }

    #[test]
    fn test_action_button_requires_action() {
        let result = ActionButtonBuilder::new("Click me")
            .icon("fas:check")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_static_page_builder() {
        let main_element = StaticPageBuilder::new()
            .title("Test Page")
            .add_paragraph(ParagraphBuilder::new().add_text("Hello").build())
            .build();

        assert_eq!(main_element.title, "Test Page");
        assert!(main_element.content.is_some());

        if let Some(main_element::Content::StaticPage(page)) = main_element.content {
            assert_eq!(page.title, "Test Page");
            assert_eq!(page.elements.len(), 1);
        } else {
            panic!("Expected StaticPage content");
        }
    }

    #[test]
    fn test_static_page_builder_with_separate_page_title() {
        let main_element = StaticPageBuilder::new()
            .title("Main Title")
            .page_title("Page Title")
            .add_paragraph(ParagraphBuilder::new().add_text("Hello").build())
            .build();

        assert_eq!(main_element.title, "Main Title");

        if let Some(main_element::Content::StaticPage(page)) = main_element.content {
            assert_eq!(page.title, "Page Title");
        } else {
            panic!("Expected StaticPage content");
        }
    }

    #[test]
    fn test_simple_table_builder() {
        let main_element = SimpleTableBuilder::new()
            .title("User Table")
            .add_header_row(
                SimpleTableRowBuilder::header()
                    .add_cell("Name")
                    .add_cell("Email")
                    .build(),
            )
            .add_data_row(
                SimpleTableRowBuilder::data()
                    .add_cell("John")
                    .add_cell("john@example.com")
                    .build(),
            )
            .build();

        assert_eq!(main_element.title, "User Table");

        if let Some(main_element::Content::SimpleTable(table)) = main_element.content {
            assert_eq!(table.rows.len(), 2);
            assert_eq!(table.rows[0].style, TableRowStyle::Header as i32);
            assert_eq!(table.rows[1].style, TableRowStyle::Data as i32);
        } else {
            panic!("Expected SimpleTable content");
        }
    }

    #[test]
    fn test_advanced_table_builder() {
        let main_element = AdvancedTableBuilder::new()
            .title("Advanced Table")
            .url("/api/data")
            .add_column(
                AdvancedTableColumnBuilder::new("id", "ID")
                    .column_type(ColumnType::Int)
                    .sortable()
                    .build(),
            )
            .add_row(AdvancedTableRowBuilder::new().add_int_cell(1).build())
            .filterable(true)
            .build();

        assert_eq!(main_element.title, "Advanced Table");

        if let Some(main_element::Content::AdvancedTable(table)) = main_element.content {
            assert_eq!(table.url, "/api/data");
            assert_eq!(table.columns.len(), 1);
            assert_eq!(table.rows.len(), 1);
            assert!(table.table_filterable);
        } else {
            panic!("Expected AdvancedTable content");
        }
    }

    #[test]
    fn test_advanced_table_without_url() {
        let main_element = AdvancedTableBuilder::new().title("Test").build();

        if let Some(main_element::Content::AdvancedTable(table)) = main_element.content {
            assert_eq!(table.url, ""); // Empty URL means repost to current URL
        } else {
            panic!("Expected AdvancedTable content");
        }
    }

    #[test]
    fn test_advanced_table_row_builder() {
        let row = AdvancedTableRowBuilder::new()
            .add_string_cell("text")
            .add_int_cell(42)
            .add_double_cell(3.14)
            .add_float_cell(2.71)
            .add_bool_cell(true)
            .build();

        assert_eq!(row.cells.len(), 5);

        if let Some(table_cell_value::Value::StringValue(s)) = &row.cells[0].value {
            assert_eq!(s, "text");
        } else {
            panic!("Expected string value");
        }

        if let Some(table_cell_value::Value::IntValue(i)) = &row.cells[1].value {
            assert_eq!(*i, 42);
        } else {
            panic!("Expected int value");
        }
    }

    #[test]
    fn test_code_block_builder() {
        let code_block = CodeBlockBuilder::new("rust")
            .add_text("fn main() {}")
            .build();

        assert_eq!(code_block.language, "rust");
        assert_eq!(code_block.text_spans.len(), 1);
    }
}
