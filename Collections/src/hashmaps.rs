use std::collections::HashMap;
pub fn run() {

    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("Satoshi"), 1); // Key: "Satoshi" and value: 1", v)
    users.insert(String::from("Mark Zuckerberg"), 2); // Key: "Mark Zuckerberg" and value: 2", v)
    users.insert(String::from("Elon Musk"), 3); // Key: "Elon Musk" and value: 3", v)

    println!("{:?}", users);

    let satoshi_value = users.get("Satoshi");
    println!("Satoshi Value: {}", satoshi_value.unwrap());
}