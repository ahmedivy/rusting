pub fn run() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }

    for number in 1..4 {
        println!("The number is: {number}");
    }

    // Reverse Range
    for number in (1..4).rev() {
        println!("The number is: {number}");
    }
}