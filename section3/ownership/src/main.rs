fn main() {
    // let var = 1;

    // let mut s = "hello".to_string(); // created on the heap

    // s.push_str(", world!");
    // println!("{}", s);

    // let x = vec!["tyler".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    let x = 1;
    let y = x; //implements copy trait
    println!("x: {}, y: {}", x, y);
}

// println!("{}", s); // produces an error
// var is dropped, s is dropped
