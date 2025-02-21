//use chrono::{Utc, Local};
use std::ops::Add;

fn main() {
    // let utc = Utc::now();
    // let local = Local::now();
    // println!("UTC: {}", utc);
    // println!("Local: {}", local);

    //

    println!("Sum: {}", sum(3, 4));

}

/// Adds two values of type T together. The type T must implement the
/// Add trait, and the Output of that Add implementation must be T.
fn sum<T: Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

