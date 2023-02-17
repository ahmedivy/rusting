fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("The value of s is '{s}'.");

    // Dangling References
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!"); // Can not do this because s is immutable
    s.len()
}

// Mutable References
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
