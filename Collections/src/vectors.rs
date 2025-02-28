pub fn run() {
    
    // Printing Vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    println!("{:?}", v);

    // using vec! macro
    let v1: Vec<i32> = vec![1,2,3];

    for i in v1 {
        println!("{}", i);
    }
}