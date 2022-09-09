fn main() {
    let length = get_length("1d".to_string());
    println!("{length}");
}

// -> return type
fn get_length(x: String) -> usize {
    x.len()
}
