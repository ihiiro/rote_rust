#[derive(Debug)]
enum NumParity {
    Even,
    Odd,
}

// enums with associated data
#[derive(Debug)]
enum Nums {
    Even(usize),
    Odd(usize),
}
// implementing a method into Nums enum
impl Nums {
    fn assign_parity(num: usize) -> Nums {
        if num % 2 == 0 {
            return Nums::Even(num);
        }

        Nums::Odd(num)
    }

    fn replace(&mut self, new_num: usize) {
        // * dereference operator
        *self = Nums::assign_parity(new_num);
    }
}

enum Animals {
    Cat,
    Cow,
    Dog,
}

impl Animals {
    fn make_sound(&self) {
        match self {
            Animals::Cat => println!("Meow!"),
            Animals::Cow => println!("Moo!"),
            Animals::Dog => println!("Woof!"),
        }
    }
}

fn multiply_by_2(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

fn main() {
    println!();
    // enum without associated data
    let one = NumParity::Odd;
    let two = NumParity::Even;
    println!("{:?}\n{:?}", one, two);

    // enums with associated data
    println!();
    let one = Nums::Odd(1);
    let two = Nums::Even(2);
    println!("{:?}\n{:?}", one, two);

    // store a number with correct parity
    println!();
    let mut one = Nums::assign_parity(3); // ::associated function
    let mut two = Nums::assign_parity(4);
    println!("{:?}\n{:?}", one, two);
    // method
    one.replace(1);
    two.replace(2);
    println!("replaced one's value with {:?} and two's with {:?}", one, two);

    // pattern matching with enums
    println!();
    let cat = Animals::Cat;
    let dog = Animals::Dog;
    let cow = Animals::Cow;
    cat.make_sound();
    dog.make_sound();
    cow.make_sound();

    // data binding
    println!();
    let five = Some(5);
    let ten = multiply_by_2(five);
    println!("five = {:?}", match five {
        None => None,
        Some(i) => Some(i),
    });
    println!("ten = {:?}", match ten {
        None => None,
        Some(i) => Some(i),
    });

    // handling one
    println!();
    let config_max = Some(1000usize);
    if let Some(max) = config_max {
        println!("maximum is configured to be {}", max);
    }
}
