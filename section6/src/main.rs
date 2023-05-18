struct Point<T> {
    // values have to be the same
    x: T,
    y: T,
}
struct Point2<T, U> {
    // values do not have to be the same
    x: T,
    y: U,
}

fn main() {
    let coord = Point { x: 5.0, y: 10.0 };
    let coord = Point2 {
        x: "Lol".to_string(),
        y: 'c',
    };
}
