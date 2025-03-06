
fn largest_i32(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];
    for i in list {
        if i > &largest {
            largest = *i;
        }
    }
    largest
}

fn largest_char(list: &Vec<char>) -> char {
    let mut largest = list[0];
    for i in list {
        if i > &largest {
            largest = *i;
        }
    }
    largest
}

// Generic Type function which can be used for any type that implements the PartialOrd trait and Copy trait.
// PartialOrd trait is used to compare two values and Copy trait is used to copy the value.
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for i in list {
        if i > &largest {
            largest = *i;
        }
    }
    largest
}
pub fn run() {
    let num_list = vec![1, 2, 3, 4, 5];
    let num_result = largest_i32(&num_list);
    println!("The largest number is {}", num_result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let char_result = largest_char(&char_list);
    println!("The largest char is {}", char_result);

    let res_num = largest(&num_list);
    println!("The largest number is {}", res_num);

    let res_char = largest(&char_list);
    println!("The largest char is {}", res_char);
}
