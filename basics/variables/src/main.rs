fn main() {
    // variable immutability
    let mut mutable_variable = 2;
    println!("\nmutable variable value is: {mutable_variable}");
    mutable_variable = 3;
    println!("mutable variable value changed to: {mutable_variable}\n");

    // constants
    const NUMBER_OF_SECONDS_IN_AN_HOUR: u32 = 60 * 60;
    println!("Number of seconds in an hour: {NUMBER_OF_SECONDS_IN_AN_HOUR}\n");

    // shadowing
    let x = 1;
    println!("x's value is: {x}");
    let x = 2;
    println!("x's value after shadowing is: {x}");
    {
        let x = 3;
        println!("x's value in inner scope is shadowed to: {x}");
    }
    println!("x's value after exiting inner scope returned to: {x}\n");

}
