mod advanced_table;
mod helpers;
mod simple_table;
mod static_page;
mod table_cell_ext;

#[cfg(test)]
mod tests;

pub use advanced_table::AdvancedTableBuilder;
pub use helpers::*;
pub use simple_table::SimpleTableBuilder;
pub use static_page::StaticPageBuilder;
pub use table_cell_ext::TableCellValueExt;
