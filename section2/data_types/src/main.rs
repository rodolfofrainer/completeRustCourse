fn main() {
    // // let x: i8 = -10;
    // // println!("The value of x is: {}", x);
    // // let y: u8 = 10; // cant be negative
    // // println!("The value of y is: {}", y);

    // // let decimal = 02_55;
    // // let hex = 0xff;
    // // let octal = 0o377;
    // // let binary = 0b1111_1111;

    // // println!("decimal: {}", decimal);
    // // println!("hex: {}", hex);
    // // println!("octal: {}", octal);
    // // println!("binary: {}", binary);

    // // let byte = b'A';
    // // println!("byte: {}", byte);

    // let x = 2.0; // f64 default
    // let y: f32 = 1.0;

    // println!("x: {}", x);
    // println!("y: {}", y);

    // let t = true;
    // let f = false;

    // println!("t: {}", t);
    // println!("f: {}", f);

    // let c = 'c';

    // println!("c: {}", c);

    // // + - * / %

    // let a = 10;
    // let b = 3;

    // let remainder = a % b;
    // let sum = a + b;
    // let minus = a - b;
    // let multiply = a * b;
    // let division = a / b;

    // println!("remainder: {}", remainder);
    // println!("sum: {}", sum);
    // println!("minus: {}", minus);
    // println!("multiply: {}", multiply);
    // println!("division: {}", division);

    //TUPLES

    // let tup = (50, "hi", true);

    // println!("tup: {:?}", tup);
    // println!("x: {}", tup.0);
    // println!("y: {}", tup.1);
    // println!("z: {}", tup.2);

    // let (x, y, z) = tup;

    // println!("x: {}", x);
    // println!("y: {}", y);
    // println!("z: {}", z);

    // ARRAYS

    // let array = [1, 2, 3];
    // for i in array {
    //     println!("i: {}", i);
    // }

    // let mut array2: [i32; 3] = [4, 5, 6];
    // println!("array2: {:?}", array2);
    // array2[0] = 10;
    // println!("array2: {:?}", array2);
    // array2[3] = 1;

    //Vectors
    // let mut nums = vec![1, 2, 3];

    // nums.push(4);
    // println!("nums: {:?}", nums);
    // nums.pop();
    // println!("nums: {:?}", nums);

    // let mut vec = Vec::new(); //vec!
    // vec.push("Test");
    // vec.push("String");
    // println!("vec: {:?}", vec);

    // vec.reverse();
    // println!("vec: {:?}", vec);

    // let mut vect = Vec::<i32>::with_capacity(2);
    // println!("{}", vect.capacity());

    // vect.push(23);
    // vect.push(3);
    // vect.push(2);
    // println!("{:?}", vect);
    // println!("{:?}", vect.capacity());

    // let v: Vec<i32> = (0..5).collect();
    // println!("{:?}", v);

    // Slices
    // let sv: &[i32] = &v[2..4];
    // println!("{:?}", sv);

    // // Strings and &str
    // let name = String::from("Rust");
    // let course = "Intro to Rust".to_string();
    // let new_name = name.replace("Rust", "Cargo");

    // println!("name: {}", name);
    // println!("course: {}", course);
    // println!("new_name: {}", new_name);

    // // &str = "string slice", "stir"
    // let _str1 = "hello"; //&str
    // println!("{}", _str1);

    // let _str2 = _str1.to_string(); //String
    // println!("{}", _str2);

    // let _str3 = &_str2;
    // println!("{}", _str3);

    // // compare strings == , !=

    // println!("{}", _str1 == _str2);
    // println!("{}", "One".to_lowercase() == "one");

    // String Literals
    // when you don't want utf-8
    // let rust = "\x52\x75\x73\x74";
    // println!("{}", rust);
}
