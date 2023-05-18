// struct Point<T> {
//     // values have to be the same
//     x: T,
//     y: T,
// }
// struct Point2<T, U> {
//     // values do not have to be the same
//     x: T,
//     y: U,
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a rust course!")
    }
}

struct Course {
    headline: String,
    author: String,
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {}
impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{},{}", self.author, self.headline)
    }
}

fn main() {
    // let coord = Point { x: 5.0, y: 10.0 };
    // let coord = Point2 {
    //     x: "Lol".to_string(),
    //     y: 'c',
    // };

    let course1 = Course {
        headline: "Rust".to_string(),
        author: "John".to_string(),
    };
    let course2 = AnotherCourse {
        headline: "Ruster".to_string(),
        author: "Jane".to_string(),
    };

    //     println!("{}", course1.overview());
    //     println!("{}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);
}

fn call_overview<T: Overview>(item: &T) {
    println!("Overview: {}", item.overview())
}
