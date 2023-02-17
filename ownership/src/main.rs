fn main() {
    // On Stack
    let name = "Ahmed";
    println!("Name: {name}");

    // On Heap (mutable)
    let city = String::from("Lahore");
    println!("City: {city}");

    {
        let country = String::from("Pakistan");
        println!("Country: {country}");
    }
    // println!("Country: {country}"); // Error: country is out of scope

    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2: {s2}");

    // s1 is moved to s2 and can no longer be used
    // Bcz both s1 and s2 are pointing to same memory 
    // location in heap, as it creates shallow copy, in
    // rust it is called as move

    // println!("s1: {s1}"); // Error: s1 is moved to s2

    // Deep Copy
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("s3: {s3}");
    println!("s4: {s4}");

    // Ownership and Functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                                // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                                // but i32 is Copy, so it's okay to still
                                                // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.