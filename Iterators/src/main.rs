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

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum(); // Takes the ownership of v2_iter, v2_iter cannot be used again
    assert_eq!(total, 6);
    println!("The sum is {}", total);

    


}
