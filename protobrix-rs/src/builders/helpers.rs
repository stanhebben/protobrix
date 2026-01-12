use crate::error::ProtobrixError;
use crate::proto::*;

/// Builder for TextSpan
#[derive(Debug, Clone, Default)]
pub struct TextSpanBuilder {
    text: String,
    color: Option<String>,
    bold: bool,
    italic: bool,
}

impl TextSpanBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
            bold: false,
            italic: false,
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn build(self) -> TextSpan {
        TextSpan {
            text: self.text,
            style: Some(TextSpanStyle {
                color: self.color.unwrap_or_default(),
                bold: self.bold,
                italic: self.italic,
            }),
        }
    }
}

/// Builder for Paragraph
#[derive(Debug, Clone, Default)]
pub struct ParagraphBuilder {
    text_spans: Vec<TextSpan>,
}

impl ParagraphBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.text_spans.push(TextSpanBuilder::new(text).build());
        self
    }

    pub fn add_span(mut self, span: TextSpan) -> Self {
        self.text_spans.push(span);
        self
    }

    pub fn add_span_builder(mut self, builder: TextSpanBuilder) -> Self {
        self.text_spans.push(builder.build());
        self
    }

    pub fn build(self) -> Paragraph {
        Paragraph {
            text_spans: self.text_spans,
        }
    }
}

/// Builder for List
#[derive(Debug, Clone, Default)]
pub struct ListBuilder {
    items: Vec<ListItem>,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, level: i32, text: impl Into<String>) -> Self {
        self.items.push(ListItem {
            level,
            text_spans: vec![TextSpanBuilder::new(text).build()],
        });
        self
    }

    pub fn add_item_with_spans(mut self, level: i32, spans: Vec<TextSpan>) -> Self {
        self.items.push(ListItem {
            level,
            text_spans: spans,
        });
        self
    }

    pub fn build(self) -> List {
        List { items: self.items }
    }
}

/// Builder for CodeBlock
#[derive(Debug, Clone, Default)]
pub struct CodeBlockBuilder {
    text_spans: Vec<TextSpan>,
    language: String,
}

impl CodeBlockBuilder {
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            text_spans: Vec::new(),
            language: language.into(),
        }
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.text_spans.push(TextSpanBuilder::new(text).build());
        self
    }

    pub fn add_span(mut self, span: TextSpan) -> Self {
        self.text_spans.push(span);
        self
    }

    pub fn build(self) -> CodeBlock {
        CodeBlock {
            text_spans: self.text_spans,
            language: self.language,
        }
    }
}

/// Builder for ActionButton
#[derive(Debug, Clone)]
pub struct ActionButtonBuilder {
    label: String,
    icon: String,
    action: Option<action_button::Action>,
}

impl ActionButtonBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: String::new(),
            action: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn go_to_url(mut self, url: impl Into<String>) -> Self {
        self.action = Some(action_button::Action::GoToUrl(GoToUrlAction {
            url: url.into(),
        }));
        self
    }

    pub fn open_page(mut self, url: impl Into<String>) -> Self {
        self.action = Some(action_button::Action::OpenPage(OpenPageAction {
            url: url.into(),
        }));
        self
    }

    pub fn open_modal(mut self, url: impl Into<String>) -> Self {
        self.action = Some(action_button::Action::OpenModal(OpenModalAction {
            url: url.into(),
        }));
        self
    }

    pub fn build(self) -> Result<ActionButton, ProtobrixError> {
        if self.action.is_none() {
            return Err(ProtobrixError::Builder(
                "ActionButton must have an action".to_string(),
            ));
        }

        Ok(ActionButton {
            label: self.label,
            icon: self.icon,
            action: self.action,
        })
    }
}

/// Builder for SimpleTableRow
#[derive(Debug, Clone, Default)]
pub struct SimpleTableRowBuilder {
    style: TableRowStyle,
    cells: Vec<SimpleTableCell>,
}

impl SimpleTableRowBuilder {
    pub fn new(style: TableRowStyle) -> Self {
        Self {
            style,
            cells: Vec::new(),
        }
    }

    pub fn header() -> Self {
        Self::new(TableRowStyle::Header)
    }

    pub fn data() -> Self {
        Self::new(TableRowStyle::Data)
    }

    pub fn add_cell(mut self, text: impl Into<String>) -> Self {
        self.cells.push(SimpleTableCell {
            text_spans: vec![TextSpanBuilder::new(text).build()],
        });
        self
    }

    pub fn add_cell_with_spans(mut self, spans: Vec<TextSpan>) -> Self {
        self.cells.push(SimpleTableCell { text_spans: spans });
        self
    }

    pub fn build(self) -> SimpleTableRow {
        SimpleTableRow {
            style: self.style as i32,
            cells: self.cells,
        }
    }
}

/// Builder for AdvancedTableColumn
#[derive(Debug, Clone)]
pub struct AdvancedTableColumnBuilder {
    id: String,
    title: String,
    description: String,
    column_type: ColumnType,
    sortable: bool,
    searchable: bool,
    filterable: bool,
    possible_values: Vec<TableCellValue>,
    decimal_digits: u32,
    unit: String,
    range_filterable: Option<RangeFilter>,
    column: Option<u32>,
    sort_direction: SortDirection,
}

impl AdvancedTableColumnBuilder {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            column_type: ColumnType::String,
            sortable: false,
            searchable: false,
            filterable: false,
            possible_values: Vec::new(),
            decimal_digits: 0,
            unit: String::new(),
            range_filterable: None,
            column: None,
            sort_direction: SortDirection::Unspecified,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn column_type(mut self, column_type: ColumnType) -> Self {
        self.column_type = column_type;
        self
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn filterable(mut self) -> Self {
        self.filterable = true;
        self
    }

    pub fn possible_values(mut self, values: Vec<TableCellValue>) -> Self {
        self.possible_values = values;
        self
    }

    pub fn decimal_digits(mut self, digits: u32) -> Self {
        self.decimal_digits = digits;
        self
    }

    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn range_filterable(mut self, min: TableCellValue, max: TableCellValue) -> Self {
        self.range_filterable = Some(RangeFilter {
            min: Some(min),
            max: Some(max),
        });
        self
    }

    pub fn column_index(mut self, index: u32) -> Self {
        self.column = Some(index);
        self
    }

    pub fn sort_direction(mut self, direction: SortDirection) -> Self {
        self.sort_direction = direction;
        self
    }

    pub fn build(self) -> AdvancedTableColumn {
        AdvancedTableColumn {
            id: self.id,
            title: self.title,
            description: self.description,
            r#type: self.column_type as i32,
            sortable: self.sortable,
            searchable: self.searchable,
            filterable: self.filterable,
            possible_values: self.possible_values,
            decimal_digits: self.decimal_digits,
            unit: self.unit,
            range_filterable: self.range_filterable,
            column: self.column.unwrap_or(0),
            sort_direction: self.sort_direction as i32,
        }
    }
}

/// Builder for AdvancedTableRow
#[derive(Debug, Clone, Default)]
pub struct AdvancedTableRowBuilder {
    cells: Vec<TableCellValue>,
    action_buttons: Vec<ActionButton>,
}

impl AdvancedTableRowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_cell(mut self, value: TableCellValue) -> Self {
        self.cells.push(value);
        self
    }

    pub fn add_string_cell(mut self, value: impl Into<String>) -> Self {
        self.cells.push(TableCellValue {
            value: Some(table_cell_value::Value::StringValue(value.into())),
        });
        self
    }

    pub fn add_int_cell(mut self, value: i32) -> Self {
        self.cells.push(TableCellValue {
            value: Some(table_cell_value::Value::IntValue(value)),
        });
        self
    }

    pub fn add_double_cell(mut self, value: f64) -> Self {
        self.cells.push(TableCellValue {
            value: Some(table_cell_value::Value::DoubleValue(value)),
        });
        self
    }

    pub fn add_float_cell(mut self, value: f32) -> Self {
        self.cells.push(TableCellValue {
            value: Some(table_cell_value::Value::FloatValue(value)),
        });
        self
    }

    pub fn add_bool_cell(mut self, value: bool) -> Self {
        self.cells.push(TableCellValue {
            value: Some(table_cell_value::Value::BooleanValue(value)),
        });
        self
    }

    pub fn add_action_button(mut self, button: ActionButton) -> Self {
        self.action_buttons.push(button);
        self
    }

    pub fn build(self) -> AdvancedTableRow {
        AdvancedTableRow {
            cells: self.cells,
            action_buttons: self.action_buttons,
        }
    }
}
