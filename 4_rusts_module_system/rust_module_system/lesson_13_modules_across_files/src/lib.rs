#![allow(dead_code)]

mod front_of_restaurant;

pub fn eat_at_restaurant() {
    front_of_restaurant::hosting::add_to_waitlist();
}
