fn main() {
    
    // Numbers
    let x: u32 = 17;
    let mut y: u32 = 188;
    println!("First val of y: {}", y);
    let f: f32 = 17.77;
    y = 18;
    println!("x: {x} f: {f} y: {y}");

    // Boolean
    let a: bool = true;
    println!("a: bool value: {a}");

    // Char
    let ch: char = 'A';
    println!("Char A: {ch}");

    // String
    let str1: &str = "Rusttt";
    
    println!("{str1}");
    let greet = String::from("Rust is best");
    //println!("{}", greet[0]); => Nahh, This is not possible in rust.

    let char1 = greet.chars().nth(2);
    // println!("Char1: {}", char1); => this is also not possible,
    println!("Char1: {}", char1.unwrap());
    /////////// OR /////////
    match char1 { // Pattern Matching
        Some(c) => println!("{}", c),
        None => println!("No Char at index 2"),
    }

    // Conditions
    let is_even: u32 = 18;
    if is_even % 2 == 0 {
        println!("Even Num: {}", is_even);
    }
    else {
        println!("Odd Num");
    }

    let condition: bool = true;
    let ans = if condition {100} else {0};
    println!("ans is: {}", ans);
    
    // Loops

    for i in 0..10 {
        print!("{} ", i);
    }
    
}

