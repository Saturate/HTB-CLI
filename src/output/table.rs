use comfy_table::{ContentArrangement, Table};

use super::Tabular;

pub fn print_table<T: Tabular>(items: &[T]) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(T::headers());

    for item in items {
        table.add_row(item.row());
    }

    println!("{table}");
}

pub fn print_key_value(fields: &[(&str, String)]) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Field", "Value"]);

    for (key, value) in fields {
        table.add_row(vec![key.to_string(), value.clone()]);
    }

    println!("{table}");
}
