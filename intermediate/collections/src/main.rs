fn main() {
    println!();
    // different ways of defining vectors
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    //
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    //
    println!("v1 = {:?} and v2 = {:?}", v1, v2);

    // accessing vector elements
    println!();
    println!("vector element at index 3 is: {}", v2[3]);
    // with get method(returns an enum)
    let third = v2.get(2);
    match third {
        Some(third) => println!("third contains: {}", third),
        None => println!("third is empty!"),
    }

    // strings
    println!();
    let s = "Some initial data";
    let mut s = s.to_string(); // to_string() and String::from() are equivalent
    s.push('h'); //only pushes a char
    println!("{s}");
    s.push_str(". There we go!;"); //can push a str (string slice)
    println!("s now contains the string: {s}");
    let s1 = String::from("Hello ");
    let s2 = String::from("there");
    let s3 = s1 + &s2;
    println!("{}", s3);
    let s4 = format!("{s2}=>{s3}");
    println!("{s4}");

    // hashmaps
    println!();
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Real Madrid", 2);
    scores.insert("Barcelona", 1);
    println!("RM {:?} - {:?} BA", match scores.get("Real Madrid") {
        Some(x) => &x,
        _ => &0, // otherwise if you want nothing to be done, return unit instead: _ => ()
    }, match scores.get("Barcelona") {
        Some(x) => &x,
        _ => &0,
    });

    println!();
    // overwriting a value in hashmap
    let mut nicknames = HashMap::new();
    nicknames.insert("Yassir", "Sierra");
    nicknames.insert("Yassir", "sierra");
    println!("{:?}", nicknames);

    // check if key already exists first
    nicknames.entry("Yassir").or_insert("SIERRA");
    nicknames.entry("yassir").or_insert("SIERRA");
    println!("{:?}", nicknames);
}
