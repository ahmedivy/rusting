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

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
