pub fn run() {
    // New scope block for expression
    let y = {
        let x = 3;
        x // Expression (no semi-colon)
    };

    println!("The value of y is: {y}.")
}