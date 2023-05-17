enum Pet {
    Cat,
    Dog,
    Bird,
    Fish,
}

impl Pet {
    fn what_am_i(&self) -> &'static str {
        match self {
            Pet::Bird => "I am a bird",
            Pet::Cat => "I am a cat",
            Pet::Dog => "I am a dog",
            Pet::Fish => "I am a fish",
        }
    }
}

enum IpAddrKind {
    V4(String),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let dog = Pet::Dog;

    println!("{}", dog.what_am_i());

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None; // Option<T> is an enum defined by the standard library

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y;

    let five = Some(5);

    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    what_pet("dog");
    what_pet("cat");
    what_pet("fish");

    let dog2 = Some(Pet::Cat);
    if let Some(Pet::Dog) = dog2 {
        println!("I am a dog");
    } else {
        println!("I am not a dog");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = 5;
    // match x {
    //     1 | 2 => println!("one or two"),
    //     _ => println!("not 1 or 2"),
    // }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = Some(5);
    let y = 5;

    match x {
        Some(10) => println!("ten"),
        Some(x) if x == y => println!("match, x = {:?}", x),
        _ => println!("no match"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet(input: &str) {
    match input {
        "cat" => println!("I am a cat"),
        "dog" => println!("I am a dog"),
        "bird" => println!("I am a bird"),
        "fish" => println!("I am a fish"),
        _ => println!("I am a pet"),
    }
}
