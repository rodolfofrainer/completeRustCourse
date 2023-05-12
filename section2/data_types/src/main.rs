fn main() {
    // let x: i8 = -10;
    // println!("The value of x is: {}", x);
    // let y: u8 = 10; // cant be negative
    // println!("The value of y is: {}", y);

    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;

    // println!("decimal: {}", decimal);
    // println!("hex: {}", hex);
    // println!("octal: {}", octal);
    // println!("binary: {}", binary);

    // let byte = b'A';
    // println!("byte: {}", byte);

    let x = 2.0; // f64 default
    let y: f32 = 1.0;

    println!("x: {}", x);
    println!("y: {}", y);

    let t = true;
    let f = false;

    println!("t: {}", t);
    println!("f: {}", f);

    let c = 'c';

    println!("c: {}", c);

    // + - * / %

    let a = 10;
    let b = 3;

    let remainder = a % b;
    let sum = a + b;
    let minus = a - b;
    let multiply = a * b;
    let division = a / b;

    println!("remainder: {}", remainder);
    println!("sum: {}", sum);
    println!("minus: {}", minus);
    println!("multiply: {}", multiply);
    println!("division: {}", division);
}
