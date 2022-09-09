fn main() {
    // tuple (explicit type annotation)
    let _tup: (f32, u32) = (1.23, 23);
    // tuple (implicit typing)
    let _tup = (1.23, 24); // underscore makes it possible to not use variable
    // pattern matching to get individual values (destructuring)
    let (x, y) = _tup;
    println!("\nfirst value of _tup is: {x}");
    println!("second value of _tup is: {y}");
    // destructuring with the . notation
    let x = _tup.0;
    let y = _tup.1;
    println!("first value of _tup is: {x}");
    println!("second value of _tup is: {y}\n");

    // chars
    let c = 's';
    println!("character is: {c}\n");

    // arrays: [types; number of elements]
    let _a: [u8; 3] = [1, 2, 3];
    // [uniform value; number of elements]
    let _a1 = [10; 10];
}
