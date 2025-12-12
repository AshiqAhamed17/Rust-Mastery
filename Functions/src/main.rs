fn main() {

     // let y = {
    //     let x = 7;
    //     x
    // };
   // println!("y: {}", y);

    func();

    // Call the f function
    println!("{}", f({ // => So the parameter is 6 to the f function
        let y = 5;
        y + 1
    }));

    /////// Closures ////////
    let initial_value: i32 = 10;
    let add = |a, b| -> i32  { a + b + initial_value};
    println!("From Add Closure: {}", add(3, 3));

    let mut count: i32 = 0;
    let mut increment = | | {
        count += 1;
        println!("Count: {}", count);
    };

    let mut add_2 = |a, b| -> i32 { a + b };
    println!("From Add 2 Closure: {}", add_2(3, 3));

    let mut decrement = | | {
        count -= 1;
        println!("Count: {}", count);
    };
    increment();
    increment();
    decrement();
    decrement();
}
