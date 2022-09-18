use std::{env, process};
use minigrep::Config;

fn main() {
    println!();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });


    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }
}