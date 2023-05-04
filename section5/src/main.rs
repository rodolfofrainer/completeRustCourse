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
}
