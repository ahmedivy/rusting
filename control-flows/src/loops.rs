pub fn run() {
    // loop {
    //     println!("Loop forever.");
    //     // break;
    // }

    let mut x = 0;
    loop {
        x += 1;
        println!("x = {x}");
        if x == 10000000 {
            break;
        }
    }
}