use crate::proto::*;

/// Builder for MainElement with AdvancedTable content
#[derive(Debug, Clone, Default)]
pub struct AdvancedTableBuilder {
    title: String,
    action_buttons: Vec<ActionButton>,
    url: String,
    columns: Vec<AdvancedTableColumn>,
    rows: Vec<AdvancedTableRow>,
    table_filterable: bool,
    name: String,
}

impl AdvancedTableBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            action_buttons: Vec::new(),
            url: String::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            table_filterable: false,
            name: String::new(),
        }
    }

    /// Set the main element title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Add an action button to the main element
    pub fn add_action_button(mut self, button: ActionButton) -> Self {
        self.action_buttons.push(button);
        self
    }

    /// Set the URL for the advanced table data endpoint (optional, defaults to current URL)
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Add a column definition
    pub fn add_column(mut self, column: AdvancedTableColumn) -> Self {
        self.columns.push(column);
        self
    }

    /// Add a data row
    pub fn add_row(mut self, row: AdvancedTableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Enable table-level filtering
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.table_filterable = filterable;
        self
    }

    /// Set the table name (used for storing user settings)
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Build the MainElement
    pub fn build(self) -> MainElement {
        MainElement {
            title: self.title,
            action_buttons: self.action_buttons,
            content: Some(main_element::Content::AdvancedTable(AdvancedTable {
                url: self.url,
                columns: self.columns,
                rows: self.rows,
                table_filterable: self.table_filterable,
                name: self.name,
            })),
        }
    }
}
