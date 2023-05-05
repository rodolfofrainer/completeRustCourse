enum Pet {
    dog,
    cat,
    fish,
}
impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::dog => "dog",
            Pet::cat => "cat",
            Pet::fish => "fish",
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
    let dog = Pet::dog;
    println!("dog: {}", dog.what_am_i());

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y; // cannot add `Option<i32>` to `i32`

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    what_pet("Dog");

    let dog2 = Some(Pet::cat);
    if let Some(Pet::dog) = dog2 {
        println!("The animal is a dog")
    } else {
        println!("Not a dog")
    };

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
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
        "Dog" => println!("I have a dog"),
        "Cat" => println!("I have a cat"),
        "Fish" => println!("I have a fish"),
        _ => println!("no clue"),
    }
}
