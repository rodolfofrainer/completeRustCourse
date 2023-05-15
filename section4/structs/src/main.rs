// struct User {
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32, i32, i32);

// struct UnitStruct;

// struct Square {
//     width: u32,
//     height: u32,
// }

// impl Square {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn whats_my_width(&self) -> u32 {
//         self.width
//     }

//     fn change_width(&mut self, width: u32) {
//         self.width = width;
//     }
// }

struct MyString<'a> {
    text: &'a str,
}

fn main() {
    let str1 = String::from("Hello");
    let x = MyString {
        text: str1.as_str(),
    };
    let s: &'static str = "I have a static lifetime.";

    //     let user1 = User {
    //         active: true,
    //         username: "Lol".to_string(),
    //         sign_in_count: 1,
    //     };
    //     println!("{}", user1.username);

    //     let user2 = build_user(String::from("thisOrThat"));
    //     println!("{}", user2.username);

    //     let coords = Coordinates(1, 2, 3);
    // }

    //     let mut square = Square {
    //         width: 5,
    //         height: 5,
    //     };

    //     println!("{}", square.area());
    //     println!("{}", square.whats_my_width());
    //     square.change_width(32);
    //     println!("{}", square.whats_my_width());

    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     } //x is dropped here

    //     println!("{}", r); // 'a

    //     //&i32
    //     //&'a i32
    //     //&'a mut i32
    // }
}
// fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// } // 'a one parameter, 'b another parameter

// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
