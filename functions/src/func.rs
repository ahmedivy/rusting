pub fn run() {
    println!("Hello, world!");
    simple_function();
    one_arg_function(5);
    multiple_arg_function(4, 'm');
    let sum = sum(5, 6);
    println!("The sum is: {sum}.");
}

fn simple_function() {
    println!("Another function.");
}

fn one_arg_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_arg_function(length: i32, unit: char) {
    println!("The length is {length} {unit}.")
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // No return keyword, No ; at the end
}