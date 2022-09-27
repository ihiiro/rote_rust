fn main() {
    println!();
    // raw pointers (unsafe)
    let mut num = 5;
    let r1 = &num as *const i32; //immutable raw pointer
    let r2 = &mut num as *mut i32; //mutable raw pointer

    // unsafe block allows dereferencing raw pointers
    unsafe {
        assert_eq!(num, *r1);
        assert_eq!(num, *r2);
        unsafe_function();
    }

    // type aliases
    type Numbers = usize;
    let _x: Numbers = 2;

    // pass function as argument to another funtion
    let num = 20;
    let factorial = factorial(return_integer, num);
    println!("factorial of {} is {}", num, factorial);
}

// immutable static variable
static _A_STATIC_VARIABLE: usize = 3;
// mutable static variable
static mut _A_MUTABLE_STATIC_VARIABLE: i32 = 3; //accessing and modifying are unsafe

// definition of unsafe function
unsafe fn unsafe_function() {}

// override the following: generic type size must be known at compile time
fn _generic_function<T: ?Sized> (_t: &T) {}

fn return_integer(i: usize) -> usize {
    i
}
fn factorial(f: fn(usize) -> usize, arg: usize) -> usize {
    let mut fact = f(arg);
    let mut arg = arg;
    while arg >= 3 {
        fact *= f(arg-1);
        arg -= 1;
    }
    fact
}
