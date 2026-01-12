use crate::proto::*;

/// Builder for MainElement with StaticPage content
#[derive(Debug, Clone, Default)]
pub struct StaticPageBuilder {
    title: String,
    action_buttons: Vec<ActionButton>,
    page_title: String,
    elements: Vec<StaticPageElement>,
}

impl StaticPageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the main element title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the static page title (defaults to main title if not set)
    pub fn page_title(mut self, page_title: impl Into<String>) -> Self {
        self.page_title = page_title.into();
        self
    }

    /// Add an action button to the main element
    pub fn add_action_button(mut self, button: ActionButton) -> Self {
        self.action_buttons.push(button);
        self
    }

    /// Add a paragraph element
    pub fn add_paragraph(mut self, paragraph: Paragraph) -> Self {
        self.elements.push(StaticPageElement {
            content: Some(static_page_element::Content::Paragraph(paragraph)),
        });
        self
    }

    /// Add a list element
    pub fn add_list(mut self, list: List) -> Self {
        self.elements.push(StaticPageElement {
            content: Some(static_page_element::Content::List(list)),
        });
        self
    }

    /// Add a simple table element
    pub fn add_simple_table(mut self, table: SimpleTable) -> Self {
        self.elements.push(StaticPageElement {
            content: Some(static_page_element::Content::SimpleTable(table)),
        });
        self
    }

    /// Add a code block element
    pub fn add_code_block(mut self, code_block: CodeBlock) -> Self {
        self.elements.push(StaticPageElement {
            content: Some(static_page_element::Content::CodeBlock(code_block)),
        });
        self
    }

    /// Build the MainElement
    pub fn build(self) -> MainElement {
        let page_title = if self.page_title.is_empty() {
            self.title.clone()
        } else {
            self.page_title
        };

        MainElement {
            title: self.title,
            action_buttons: self.action_buttons,
            content: Some(main_element::Content::StaticPage(StaticPage {
                title: page_title,
                elements: self.elements,
            })),
        }
    }
}
