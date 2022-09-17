pub fn _silly_function(i: u8) -> i32 {
    println!("Got the number {}", i);
    49
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() {
        assert_eq!(49, _silly_function(23));
    }
}
