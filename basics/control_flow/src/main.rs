fn main() {
    let condition = false;
    let x = if condition {1} else {0};
    println!("\nThe value of x is: {x}");

    // forever loop
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            println!("\nvalue of x is now: {x}\n");
            break;
        }
    }

    // loop labels
    let mut x = 0;
    '_outer: loop {
        println!("execution is in outer loop.");
        '_inner: loop {
            println!("execution is in inner loop.");
            x += 1;
            if x == 5 {
                break '_outer;
            }
        }
    }

    // while loop
    println!();
    while x != 0 {
        println!("{x}");
        x -= 1;
    }
    println!("IGNITION!\n");

    // for loop
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for element in a {
        println!("{element}");
    }

    // for loop and a Range
    println!();
    // (inclusive, not inclusive)
    for number in (1..4).rev() { // .rev() reverses the range
        println!("{number}");
    }
    println!("IGNITION!");
}
