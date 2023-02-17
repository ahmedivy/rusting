pub fn run() {
    'outer: loop {
        println!("Outer loop.");
        'inner: loop {
            println!("Inner loop.");
            break 'outer;
        }
        println!("This will never print.");
    }
    println!("Exited outer loop.");
}