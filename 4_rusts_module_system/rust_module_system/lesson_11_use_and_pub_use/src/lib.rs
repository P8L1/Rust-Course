#![allow(dead_code)]

pub mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_restaurant::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
