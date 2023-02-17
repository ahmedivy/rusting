pub fn run() {
    // loop {
    //     println!("Loop forever.");
    // }

    let mut x = 0;
    loop {
        x += 1;
        println!("x = {x}");
        if x == 10 {
            break;
        }
    }

    // Return value from loop
    let result = loop {
        x += 1;
        println!("x = {x}");
        if x == 20 {
            // Expression after break is the return value
            break x * 2;
        }
    };
    println!("Result: {result}");
}