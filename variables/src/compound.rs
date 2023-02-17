use std::io;

pub fn run() {
    // Tuple
    let tup = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    let (x, _y, _z) = tup;
    println!("The value of x is: {}", x);
    // With type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
    let first = arr[0];
    println!("The value of first is: {}", first);
    let arr = [3; 5];
    println!("The value of arr is: {:?}", arr);
    // With type annotation
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);

    // Invalid Array Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}.");
}