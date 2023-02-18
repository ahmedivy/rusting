struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut ahmed = build_user(String::from("ahmad@mail.com"), String::from("ahmed"));

    ahmed.email = String::from("ahmed@gmail.com");

    println!("{}'s email is {}", ahmed.username, ahmed.email);
    println!(
        "{} is {}",
        ahmed.username,
        (if ahmed.active { "Active" } else { "Inactive" })
    );
    println!("{} signed in {} times", ahmed.username, ahmed.sign_in_count);

    // Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black is ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin is ({}, {}, {})", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
