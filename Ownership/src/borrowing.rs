
pub fn run() {
    
    let my_str = String::from("My String-1");
    let my_str2 = &my_str; // Reference to my_str
    borrow(my_str2); // Passing the reference to borrow fn
    println!("From Owner: {}", my_str); // This is still valid becoz ownership is just borrowed

    ////// Mutable Borrowing //////
    let mut str = String::from("Hello, ");
    println!("From Owner - Before Update: {}", str);

    mut_borrow(&mut str);
    println!("From the Owner - After: {}", str);
    
    let s3 = &mut str;
    println!("s3: {}", s3);
    
    let s4 = &mut str;
    println!("s4: {}", s4);
    // println!("s3: {}", s3); ->  Error: This cannot happen as only one borrower can be at a time


}

fn borrow(str: &String) {
    println!("String from the borrow fn: {}", str);
}

// Borrowing and updating the str.
fn mut_borrow(str: &mut String) {
    str.push_str("World");
    println!("From the mut_borrow fn: {}", str);

}