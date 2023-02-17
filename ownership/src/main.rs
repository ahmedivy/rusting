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

}
