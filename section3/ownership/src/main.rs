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

    // let x = 1;
    // let y = x; //implements copy trait
    // println!("x: {}, y: {}", x, y);

    // let s = String::from("takes"); // create a variable with a string "takes"
    // takes_ownership(s); // give ownership to function

    // let val = 1;
    // makes_copy(val);

    // let s1: String = give_ownership();
    // println!("{}", s1);

    // let str3: String = take_and_give(s1);

    // if true {
    //     let str4 = str3;
    // } else {
    //     let str5 = str3;
    // }

    // let mut str1 = String::from("hello");
    // let mut str2: String;

    // loop {
    //     str2 = str1;
    // }

    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn take_and_give(str2: String) -> String {
//     str2
// }

// fn give_ownership() -> String {
//     "given...".to_string()
// }

// fn makes_copy(i: i32) {
//     let val1 = i;
//     println!("{}", i);
// }

// fn takes_ownership(s: String) {
//     let strin = s;
//     println!("{}", strin);
// }

// println!("{}", s); // produces an error
// var is dropped, s is dropped
