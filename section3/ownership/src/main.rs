fn main() {
    // let var = 1;

    // let mut s = "hello".to_string(); // created on the heap

    // s.push_str(", world!");
    // println!("{}", s);

    let x = vec!["tyler".to_string()];
    let y = x;

    // println!("{:?}", x); // produces error: borrow of moved value: `x`
}

// println!("{}", s); // produces an error
// var is dropped, s is dropped
