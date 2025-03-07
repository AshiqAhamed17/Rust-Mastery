use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}

pub fn run() {
    let user = User {
        name: String::from("John"),
        age: 25,
    };

    // Serialize the User struct to a JSON string
    let json_data = serde_json::to_string(&user);
    match json_data {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error: {}", e),
    }
    
    
}