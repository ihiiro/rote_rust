mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_meal() {}

pub mod work {
    pub fn do_some_work() {
        super::deliver_meal();
    }
}

// create alias
// also reexporting it with pub (can be used by others)
pub use crate::work::do_some_work as do_work;

// nested paths
// instead of this
use std::cmp::Ordering;
use std::io;
// do this
use std::{cmp::Ordering, io};
// also
use std::io;
use std::io::Write;
// this instead
use std::io::{self, Write};
// bring all public items into scope
use std::collections::*;
