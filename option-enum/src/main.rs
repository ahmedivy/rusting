fn main() {
    // Rust does not have nulls, but it does
    // have an enum that can encode this concept.
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
