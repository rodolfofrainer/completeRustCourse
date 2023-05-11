// Generic struct
// struct Point<T, U> {
//     x: T, //i32
//     y: U, //i32
// }

struct Car {
    mpg: i8,
    color: String,
    top_speed: i16,
}

#[derive(Debug)]
struct Motorcycle {
    mpg: i8,
    color: String,
    top_speed: i16,
}

pub trait Properties {
    fn set_mpg(&mut self, mpg: i8);
    fn set_color(&mut self, color: String);
    fn set_top_speed(&mut self, top_speed: i16);
}

impl Properties for Car {
    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed;
    }
}

impl Properties for Motorcycle {
    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed;
    }
}

fn print<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

// Traits
// trait Overview {
//     fn overview(&self) -> String {
//         String::from("This is a rust course")
//     }
// }

// trait Clone: Sized {
//     fn clone(&self) -> Self;
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone();
//     }
// }

// struct Course {
//     headline: String,
//     author: String,
// }

// struct AnotherCourse {
//     headline: String,
//     author: String,
// }

// impl Overview for Course {}

// impl Overview for AnotherCourse {
//     fn overview(&self) -> String {
//         format!("{}, {}", self.author, self.headline)
//     }
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Add for Point<T>
// where
//     T: Add<Output = T>,
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

fn main() {
    let mut motorcycle = Motorcycle {
        mpg: 0,
        color: String::from("red"),
        top_speed: 0,
    };

    motorcycle.set_mpg(100);
    motorcycle.set_color(String::from("blue"));
    motorcycle.set_top_speed(200);

    println!("{:?}", motorcycle);
    println!("{:?}", motorcycle.mpg);
    println!("{:?}", motorcycle.color);
    println!("{:?}", motorcycle.top_speed);

    print(vec![1, 2, 3]);
    print("String");
    print(String::from("String 2"));
    print(10);

    // let coord = Point { x: 5.0, y: 5.0 };
    // let coord2 = Point { x: 1.0, y: 2.0 };

    // let sum = coord + coord2;
    // println!("{:?}", sum);

    // let coord = Point { x: 5.0, y: 5.0 };
    // let coord2 = Point { x: 'x', y: 'y' };
    // let coord3 = Point {
    //     x: 'x',
    //     y: String::from("LOL"),
    // };

    // let course1 = Course {
    //     headline: String::from("Headline"),
    //     author: String::from("Tyler"),
    // };

    // let course2 = AnotherCourse {
    //     headline: String::from("Headline2"),
    //     author: String::from("NotTyler"),
    // };

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);
} //course1 went out of scope

// fn call_overview<T: Overview>(item: &T) {
//     println!("Overview: {}", item.overview());
// }

// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T:Overview>(item1:&T, item2:&T)
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T:Overview + AnotherTrait>(item1: &T, item2: &T)

// impl Drop for Course {
//     fn drop(&mut self) {
//         println!("Dropping {}", self.author);
//     }
// }
