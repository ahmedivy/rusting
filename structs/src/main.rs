struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

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
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
