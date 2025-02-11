mod borrowing;

fn take_ownership(s: String) {
    println!("Inside function: {}", s);
}

fn give_ownership(s: String) -> String {
    println!("From give_ownership func: {}", s);
    s // Ownership returns to the caller
}

fn main() {
    
    let s1 = String::from("Hello Rust s1"); // s1 owns the string in the heap memory
    println!("From s1(Old Owner): {}", s1);

    let s2 = s1; // Ownership moves from s1 form s2
    //println!("{}", s1); => Error: This is not possible because s1 is no longer valid
    println!("From s2(New Owner): {}", s2); // Works Fine

    let s3 = s2.clone(); // Creates a copy
    println!("s2: {}", s2); // Works Fine
    println!("s3 cloned one: {}", s3);

    //////// Function taking Ownership //////////
    let s4 = String::from("Rust s4");
    take_ownership(s4);
    
    //println!("{}", s);  => Error: s is no longer valid coz the ownership has been moved inside the take_ownership function.

    ////////// Returning Ownership /////////
    let mut s5 = String::from("Rust s5");
    println!("{}", s5);
    s5 = give_ownership(s5);
    println!("s5: {}", s5);

    println!("-------------- FROM BORROWING fn -----------------");
    borrowing::run();

}