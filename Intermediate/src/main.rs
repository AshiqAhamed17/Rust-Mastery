
// struct User {
//     active: bool,
//     name: String,
//     email: String,
//     count: u32
// }

struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width //  same as -> return self.height * self.width;
    }
}
fn main() {
    // let mut user1 = User {
    //     active: true,
    //     name: String::from("Justin"), // Short hand syntax -> name: name
    //     email: String::from("jus@gmail.com"), // Short hand syntax -> name: name
    //     count: 1,
    // };

    let rect1 = Rect {
        height: 5,
        width: 2,
    };

    println!("The area of the rectangle is: {}", rect1.area());
    
}
