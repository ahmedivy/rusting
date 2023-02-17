fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");
    // let slice = &s[0..2];
    // let slice = &s[..2];
    // let slice = &s[3..];
    let slice = &s[..];

    println!("{slice}");
}
