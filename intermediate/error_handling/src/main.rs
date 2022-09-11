use std::fs::File;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!();
    // panic! macro makes the program panic
    // panic!("The program is fucking panicking!!!!");

    let namesfile_result = File::open("names.txt");

    let _namesfile = match namesfile_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("names.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    // the unwrap method
    println!();
    let _numbersfile = File::open("numbers.txt").unwrap();

    // expect method
    println!();
    let _charsfile = File::open("chars.txt").expect("Are you stupid or smt? no file exists!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("usernames.txt")
}
