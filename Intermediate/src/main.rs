//mod enums;

// struct User {
//     active: bool,
//     name: String,
//     email: String,
//     count: u32
// }


fn main() {
    
    //enums::run();

    let ans = fib(10);
    println!("{}", ans);

    let my_str = String::from("ZKP is so cool");
    let str_count = count_str(&my_str);
    println!("String count: {}", str_count);
    
}

fn count_str(s: &str) -> usize {
    s.chars().count()
}

fn fib(num: u32) -> u32 {
    
    let mut first = 0;
    let mut sec = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return 1;
    }

    for _ in 1..num-1 {
        let temp = sec;
        sec = first + temp;
        first = temp;
    }

    return sec;

}
