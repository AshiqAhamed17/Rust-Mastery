
pub fn run() {
    
    let my_str = String::from("My String-1");
    let my_str2 = &my_str; // Reference to my_str
    borrow(my_str2); // Passing the reference to borrow fn
    println!("From Owner: {}", my_str); // This is still valid becoz ownership is just borrowed

}

fn borrow(str: &String) {
    println!("String from the borrow fn: {}", str);
}