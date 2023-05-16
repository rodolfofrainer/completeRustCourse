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
}
