// use std::{fmt::format, path::Display};
// trait Shape {
//     fn area(&self) -> f32;
// }

// struct Rect {
//     width: f32,
//     height: f32
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height
// 	}
// }

// fn get_area_2<T: Shape>(shape: T) -> f32 {
//     return shape.area()
// }


///////// MACROS //////////

// Create a macro that prints "Hello Rust Programmers"
macro_rules! say_hello {
    () => {
        println!("Hello Rust Programmers");
    };
}

// Defining a create_function macro
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name))
        }
    };
}

create_function!(hello_func); // This will create a function called "hello"
create_function!(ash);


#[derive(Debug)]
struct User {
	username: String,
	password: String,
	age: u32
}
fn main() {
    
    say_hello!();
    hello_func();
    ash();

    let u = User{
        username: String::from("ABC"),
        password: String::from("ads"),
        age: 19
    };

    println!("{:?}", u);
    
    
}
