struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!();

    // literals and catch all _ pattern
    let x = 5;
    match x {
        5 => println!("Lucky number 5"),
        _ => println!("Some other number"),
    }

    // shadowing
    // x value inside match block doesn't match x value in the outer scope
    println!();
    let y = Some(10);
    match y {
        // Some(x) matches any value of that isn't None
        Some(x) => println!("Got the shadowed value x = {x} from inside match expression"),
        _ => return,
    }
    println!("Value of outer x = {x}");

    // OR pattern
    println!();
    let x = 6;
    match x {
        1 | _ => println!("Matches anything"),
    }

    // range pattern
    println!();
    let x = 10;
    let y = 5;
    match x {
        1..=9 => println!("{x} in [ 1 ; 9 ]"),
        1..=10 => println!("{x} [ 1 ; 10 ]"),
        _ => println!("{x} in ] -inf ; +inf ["),
    }
    match y {
        1..=9 => println!("{y} in [ 1 ; 9 ]"),
        1..=10 => println!("{y} in [ 1 ; 10 ]"),
        _ => println!("{y} in ] -inf ; +inf ["),
    }

    // range of char values
    let y = 'b';
    match y {
        'a'..='d' => println!("between a and d"),
        _ => println!("some other range"),
    }

    let x = Point { x: 0, y: 1 };
    // destructuring pattern in match expression
    println!();
    match x {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x},{y})"),
    }

    // destructuring structs
    // creates new variables x and y and places appropriate value for each using the pattern
    // x gets 0 and y gets 1. Also this is shorthand for x: x, y: y and difference variable names
    // can be used too but this is better
    let Point { x, y } = x;
    assert_eq!(0, x);
    assert_eq!(1, y);

    // ignoring with ..
    println!();
    let my_tuple = (1, "Hello", false, Some(69));
    match my_tuple {
        (x, ..) => println!("first value is: {x}"),
    }

    // match guards
    println!();
    let list_of_numbers = (Some(1), Some(2), Some(3));

    match list_of_numbers {
        (_, Some(x), ..) if x % 2 == 0 => println!("{x} is even"),
        _ => return,
    }

    // @ symbol for saving value while testing
    let p = Point { x: 2000394, y: 1 };
    match p {
        // binds whatever the actual value is in x to a variable named x
        Point { x: x @ 0.., .. } => println!("x is atleast 0: x = {x}"),
        _ => return
    }

}
