pub mod json;
pub mod table;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}

pub trait Tabular {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}

pub fn print_list<T: serde::Serialize + Tabular>(items: &[T], format: OutputFormat) {
    match format {
        OutputFormat::Table => table::print_table::<T>(items),
        OutputFormat::Json => json::print_json(items),
    }
}

pub fn print_detail<T: serde::Serialize>(
    item: &T,
    format: OutputFormat,
    fields: &[(&str, String)],
) {
    match format {
        OutputFormat::Table => table::print_key_value(fields),
        OutputFormat::Json => json::print_json(item),
    }
}

pub fn print_message(msg: &str) {
    println!("{msg}");
}

pub fn print_pagination(current: u32, total_pages: u32, total_items: u32) {
    if total_pages > 1 {
        println!("Page {current} of {total_pages} ({total_items} total)");
    }
}
