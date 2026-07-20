pub fn print_json<T: serde::Serialize + ?Sized>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Failed to serialize JSON: {e}"),
    }
}
