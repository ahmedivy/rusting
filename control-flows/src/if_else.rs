pub fn run() {
    
    // If-else
    let age = 18;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You can not vote.");
    }

    // let x = 0;
    // Rust will not convert zero and one to boolean like in Ruby or Python
    // if x {
    //     println!("x is true.");
    // }
    
    // Shorthand if
    let is_of_age : bool = if age >= 18 { true } else { false };
    println!("Is of age: {is_of_age}."); // Is of age: true.
}
