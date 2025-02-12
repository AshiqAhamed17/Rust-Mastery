
struct User {
    active: bool,
    name: String,
    email: String,
    count: u32
}
fn main() {
    //let mut name = String::from("Justin");
    //let mut email = String::from("jsutin@gmail.com");

    let mut user1 = User {
        active: true,
        name: String::from("Justin"), // Short hand syntax -> name: name
        email: String::from("jus@gmail.com"), // Short hand syntax -> name: name
        count: 1,
    };

    println!("Name: {}", user1.name);
    println!("email: {}", user1.email);

    user1.name = String::from("Jsutin Bieber");

    println!("Updated Name: {}", user1.name);
}
