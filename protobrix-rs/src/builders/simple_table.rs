use crate::proto::*;

/// Builder for MainElement with SimpleTable content
#[derive(Debug, Clone, Default)]
pub struct SimpleTableBuilder {
    title: String,
    action_buttons: Vec<ActionButton>,
    rows: Vec<SimpleTableRow>,
}

impl SimpleTableBuilder {
    pub fn new() -> Self {
        Self::default()
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

    /// Add a header row
    pub fn add_header_row(mut self, row: SimpleTableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Add a data row
    pub fn add_data_row(mut self, row: SimpleTableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Add any row
    pub fn add_row(mut self, row: SimpleTableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Build the MainElement
    pub fn build(self) -> MainElement {
        MainElement {
            title: self.title,
            action_buttons: self.action_buttons,
            content: Some(main_element::Content::SimpleTable(SimpleTable {
                rows: self.rows,
            })),
        }
    }
}
