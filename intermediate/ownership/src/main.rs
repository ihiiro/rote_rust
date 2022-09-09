fn main() {
    println!();
    // mutable string(requests data to be allocated from the heap)
    let mut s = String::from("Yassir");
    s.push_str(" is me."); // append
    println!("{s}");

    // s1 is dropped when s2 points to the same data
    println!();
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{s2}");
    // println!("{s1}"); can't be used (returns error)

    // cloning data
    println!();
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1} and s2 = {s2}");

    // references
    println!();
    let s1 = String::from("Hi");
    let length = get_length(&s1);
    println!("Length of {s1} is {length}");

    // modifying a String by using a mutable reference
    let mut s1 = String::from("Hi");
    println!();
    println!("s1 pre-modification: {s1}");
    modify_str(&mut s1);
    println!("s1 post-modification: {s1}");

    // the Slice type
    println!();
    let s1 = String::from("Hello there!");
    // equals [0..5]
    let hello = &s1[..5];
    // equals [6..length of string]
    let there = &s1[6..];
    println!("s1 = {s1} and hello = {hello} and there = {there}");
    let first_word = first_word(&s1);
    println!("The first word in {s1} is {first_word}");
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn modify_str(s: &mut String) { // &mut is a mutable reference
    s.push_str("_Modified by function.");
}

// missing lifetime specifier error
// fn dangle() -> &String {
//     let str = "sdfkjfl";
//
//     str
// }

fn first_word(s: &str) -> &str { // &str is the String slice type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
