mod generics;
mod serde;
mod lifetimes;
fn main() {

    println!("---------From Generics Module---------");
    generics::run();

    println!("---------From Serde Module---------");
    serde::run();

    println!("---------From Lifetimes Module---------");
    lifetimes::run();

}
