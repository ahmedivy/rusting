#[derive(Debug)]
enum Message {
    Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

pub fn run() {
    let m = Message::Write(String::from("hello"));
    m.call();
    // let m = Message::Move { x: 3, y: 4 };
    // println!("m: {:?}", m);
    // m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::ChangeColor(0, 160, 255);
    m.call();
}
