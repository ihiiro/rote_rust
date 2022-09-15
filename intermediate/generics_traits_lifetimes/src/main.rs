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

// defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub body: String,
    pub id: i32,
}

// implementing the trait on Article struct
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}: {}...", self.title, self.author, &self.body[..29])
    }
}

// implementing Summary trait for a function's parameters
// multiple traits can be added with +
// example: fn summarize(item: &(impl Summary + Display)) -> String
fn summarize(item: &impl Summary) -> String {
    format!("Summary:\n|{}|", item.summarize())
}
// summarize can also be written with trait bound syntax
// multiple traits can be added with + too
// example: fn summarize2<T: Summary + Display>(item: &T) -> String
fn _summarize2<T: Summary>(item: &T) -> String {
    format!("Summary:\n|{}|", item.summarize())
}
// summarize3 uses the where clause to declutter the function's signature and make it readables
// used when there are more than one generic type parameters
fn _summarize3<T>(item: &T) -> String
    // where generic type parameter: trait,
             // ...: ..
    where T: Summary,
{
    format!("Summary:\n|{}|", item.summarize())
}

// generic lifetime parameter: convention is lowercase and short after the ' inside <>
// note: generic type parameters can be added inside the same <>, example: <'a, T>
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
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

    //
    println!();
    let new_article = Article {
        title: String::from("The ways of the man"),
        author: String::from("PewDiePie"),
        body: String::from("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
        quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat
        nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia
        deserunt mollit anim id est laborum."),
        id: 1,
    };

    println!("article[ID]={}::{}", new_article.id, new_article.summarize());

    //
    println!();
    println!("{}", summarize(&new_article));

    // explicit lifetimes: generic lifetime parameters
    println!();
    let x = "sdjfdlsfjldfjdlfkjdf";
    let y = "kdfjdkln";
    println!("largest string is: {}", longest(&x, &y));
}
