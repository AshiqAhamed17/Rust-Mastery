

pub fn run() {
   
    let mut v =  Vec::new();
    v.push(1);
    v.push(2);
    v.push(6);
    v.push(4);

    let v3: Vec<i32> = vec![3, 81, 16, 63];

    let v2: Vec<i32> = even(&v);

    println!("{:?}",v);
    println!("{:?}", v2);
    println!("{:?}", v3);


}


fn even(v: &Vec<i32>) -> Vec<i32> {
    let mut v2: Vec<i32> = Vec::new();

    for val in v {
        if val % 2 == 0 {
            v2.push(*val);
        }
    }

    return v2;

}