#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

use std::ops::Deref;

struct MyBox<T>(T); //syntax meaning that this is a tuple struct with one element, hence: (T)
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// implementing Deref trait for MyBox<T>: now the enum can be dereferenced like a regular reference
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}
// implementing Drop trait for CustomSmartPointer
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

use crate::List::{Cons, Nil};

fn main() {
    println!();
    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive data structure: {:?}", cons_list);

    //
    let x = 2;
    let y = MyBox::new(x);

    assert_eq!(2, x);
    assert_eq!(2, *y); // assert_eq!(2, y) would throw an error: can't compare {integer} with &{integer}

    //
    println!();
    // after _some_data and _some_data2 go out of scope, the drop method will be called automatically
    let _some_data = CustomSmartPointer { data: String::from("I am data!") };
    drop(_some_data); // drop early
    let _some_data2 = CustomSmartPointer { data: String::from("I am data too!") };

    // reference counting smart pointer Rc<T>
    println!();
    use crate::ListRc::{ConsRc, NilRc};

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("reference count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("reference count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("reference count after creating c = {}", Rc::strong_count(&a));
    }
    println!("reference count after c goes out of scope = {}", Rc::strong_count(&a));
}
