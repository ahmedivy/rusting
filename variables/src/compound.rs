pub fn run() {
    // Tuple
    let tup = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    let (x, _y, _z) = tup;
    println!("The value of x is: {}", x);
}