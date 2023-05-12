fn main() {
    // let one = 1;
    // if one > 10 {
    //     println!("{} > 0", one);
    // } else if one == 1 {
    //     println!("equal");
    // } else {
    //     println!("false");
    // }

    // let mut num = 0;
    // 'counter: loop {
    //     println!("Count: {}", num);
    //     let mut decrease = 5;
    //     'inner: loop {
    //         println!("Decrease: {}", decrease);
    //         if decrease == 4 {
    //             break 'inner;
    //         }
    //         if num == 2 {
    //             break 'counter;
    //         }
    //         decrease -= 1;
    //     }
    //     num += 1;
    // }

    // let mut counter = 0;
    // let limit = 100;
    // while counter <= limit {
    //     println!("Count: {}", counter);
    //     counter += 1;
    // }

    // let vec: Vec<i8> = (0..10).collect();
    // for i in vec.iter() {
    //     println!("i: {}", i);
    // }

    for number in (1..5).rev() {
        println!("{}!", number);
    }
}
