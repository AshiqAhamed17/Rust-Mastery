use std::collections::HashMap;

fn main() {
    let mut v1 = vec![1, 2, 3];

    //* Simple way to iterate using for loops
    // for val in v1 { // Moves ownership (invalidates v1) it explicitly moves v1 into v1.into_iter()
    //     println!("{}", val);
    // }

    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    //* To borrow the variables mutable (Mutable iterator)
    let  mut_iter = v1.iter_mut();
    for val in mut_iter {
        *val = *val + 1;
        println!("{}", val);
    }

    //* using while loop
    // while let Some(val) = mut_iter.next() {
    //     println!("{}", val)
    // }

    //* into_iter() => It takes ownership of the collection
    let iter_into = v1.into_iter();
    for val in iter_into {
        println!("{}", val);
    }

    let v2 = vec![1, 2, 3, 4];
    let v2_iter = v2.iter(); //// Takes the ownership of v2_iter, v2_iter cannot be used again
    //let total: i32 = v2_iter.sum();
    //assert_eq!(total, 10, "Total is incorrect");
    let iter2 = v2_iter.map(|x| x * 2);
    for i in iter2 {
        println!("map vals: {}", i);
    }

    println!("{:?}", v2);



    // * Filter all odd values then double each value and create a new vector.
    let vect = vec![1, 2, 3, 4, 5];
    let ans = filter_and_map(vect);
    println!(" Filter and map: {:?}", ans);

    // * Iterators over Hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let map_iter = scores.iter();
    for (key, value) in map_iter {
        println!("{}: {}", key, value);
    }


}

fn filter_and_map(v: Vec<i32>) -> Vec<i32> {
    let ans_vec: Vec<i32>;
    let iter = v.iter();

    let iter2 = iter.filter(|x| *x% 2 == 1).map(|x| x * 2);

    ans_vec = iter2.collect();

    ans_vec

}

