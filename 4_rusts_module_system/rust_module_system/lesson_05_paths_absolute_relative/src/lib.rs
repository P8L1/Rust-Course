#![allow(dead_code)]

pub mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_restaurant::hosting::add_to_waitlist();
    front_of_restaurant::hosting::add_to_waitlist();
}
