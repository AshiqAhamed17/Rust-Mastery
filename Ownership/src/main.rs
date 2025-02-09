fn main() {
    
    let s1 = String::from("Hello Rust"); // s1 owns the string in the heap memory
    println!("From s1(Old Owner): {}", s1);

    let s2 = s1; // Ownership transferred  to s2 form s1
    //println!("{}", s1); => Error: This is not possible because s1 is no longer valid
    println!("From s2(New Owner): {}", s2);
}