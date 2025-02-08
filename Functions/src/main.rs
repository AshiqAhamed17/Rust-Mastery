fn main() {
    func();

    // let y = {
    //     let x = 7;
    //     x
    // };
   // println!("y: {}", y);

    println!("{}", f({ // => So the parameter is 6 to the f function
        let y = 5;
        y + 1
    }));
}


fn func() {
    println!("From func Function");
}


fn f(x: i32) -> i32 {x + 1}


