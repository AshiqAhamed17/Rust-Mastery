use std::collections::HashMap;

fn group_values(v: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm: HashMap<String, i32> = HashMap::new();

    for (key, val) in v {
        hm.insert(key, val);
    }
    return hm;
}

pub fn run() {

    let mut users: HashMap<String, i32> = HashMap::new();

    users.insert(String::from("Satoshi"), 1); // Key: "Satoshi" and value: 1")
    users.insert(String::from("Mark Zuckerberg"), 2); // Key: "Mark Zuckerberg" and value: 2")
    users.insert(String::from("Elon Musk"), 3); // Key: "Elon Musk" and value: 3")

    let v = vec![(String::from("ASH"), 17), (String::from("AQ"), 77)];


    let ans: HashMap<String, i32> = group_values(v);
    println!("{:?}", ans);

    println!("{}", ans.get("ASH").unwrap());

    println!("{:?}", users);

    let satoshi_value = users.get("Satoshi");
    println!("Satoshi Value: {}", satoshi_value.unwrap());
}