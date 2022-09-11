// <generic type: only to types that implement some trait(PartialOrd in this example)>
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// struct fields can have any type T ("T" is conventional shorthand for "type")
#[derive(Debug)]
struct Point<T> {
    _x: T, //_ is so that I don't have to use them
    _y: T,
}

// multiple generic type parameters
#[derive(Debug)]
struct Pair<T, U> {
    _x: T,
    _y: U,
}

// in method definitions too
// the <T, U> after impl tells rust that the types in Pair<T, U> are generic
impl<T, U> Pair<T, U> {
    fn return_pair(&self) -> (&T, &U) {
        (&self._x, &self._y)
    }
}

// defining a method on specific instances (pairs with a &str and an i32 in this case)
impl Pair<&str, i32> {
    fn describe(&self) {
        println!("{} has value {}", &self._x, &self._y);
    }
}

fn main() {
    println!();
    let list_of_chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    println!("Largest char in list_of_chars is {}", largest(&list_of_chars));

    println!();
    let list_of_numbers = vec![232, 343, 344, 3958];

    println!("Largest number in list_of_numbers is {}", largest(&list_of_numbers));

    println!();
    let integer = Point { _x: 1, _y: -2 };
    let float = Point { _x: 23.27, _y: 232.9 };
    let chars = Point { _x: 'w', _y: 'e' }; //even chars work

    println!("integer {:#?}\nfloat {:#?}\nchars {:#?}", integer, float, chars);

    //
    println!();
    let pair = Pair { _x: "Yassir", _y: 19 };
    println!("pair {:#?}", pair);

    println!();
    println!("pair {:?}", pair.return_pair());
    pair.describe();

    //
    println!();
    let integer_and_opposite = Pair { _x: 1, _y: -1 };
    println!("integer_and_opposite {:?}", integer_and_opposite.return_pair());
}
