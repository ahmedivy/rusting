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
}
