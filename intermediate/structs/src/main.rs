// implement Debug trait
#[derive(Debug)]
struct Order {
    order: String,
    location: String,
    queue_number: u64,
}

// tuple struct
struct Passcodes(u32, u32, u32);

// structs with methods (associated functions)
impl Order {
    fn deliver_order(&self) {
        println!("{} delivered.", self.order);
    }

    fn default_order() -> Self {
        Self {
            order: String::from("Hotdogs"),
            location: String::from("USA"),
            queue_number: 0,
        }
    }
}

// unit-like structs
struct _Something;

fn main() {
    println!();
    let my_order = place_order(String::from("Pizza"), String::from("Rabat"));
    println!("order: {},\nlocation: {},\nqueue_number: {}",
             my_order.order, my_order.location, my_order.queue_number);

    // struct update syntax
    println!();
    let my_other_order = Order {
        queue_number: 2,
        ..my_order // same remaining field values as my_order
    };
    println!("order: {},\nlocation: {},\nqueue_number: {}",
             my_other_order.order, my_other_order.location, my_other_order.queue_number);
    // my_order is no longer valid because we moved values into my_other_order without Copy trait
    // println!("{}", my_order.order);

    // tuple structs
    println!();
    let passcodes = Passcodes(12343434, 34374384, 34983745);
    println!("{}, {}, {}", passcodes.0, passcodes.1, passcodes.2);

    // {:?} and {:#?} are for debugging display for types that implement Debug trait
    println!();
    let data_tuple = ("data", 'c');
    println!("{:#?}", data_tuple);
    println!("{:?}", data_tuple);

    // the following code requires that Order implement the Debug trait
    println!();
    println!("{:?}", my_other_order);

    // dbg!
    println!();
    dbg!(&my_other_order);

    // structs with methods
    println!();
    my_other_order.deliver_order();

    // using a constructor associated function
    println!();
    let new_order = Order::default_order();
    println!("{:?}", new_order);
}

fn place_order(order: String, location: String) -> Order {
    // using the field init shorthand
    Order {
        order,
        location,
        queue_number: 1,
    }
}
