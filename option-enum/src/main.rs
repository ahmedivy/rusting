// The Option<T> enum is defined as follows:
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // Rust does not have nulls, but it does
    // have an enum that can encode this concept.
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_string, absent_number);
}
