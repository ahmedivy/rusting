fn main() {
    println!("Hello, world!");
    simple_function();
    one_arg_function(5);
    multiple_arg_function(4, 'm');
}

fn simple_function() {
    println!("Another function.");
}

fn one_arg_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_arg_function(length: i32, unit: char) {
    print!("The length is {length} {unit}.")
}
