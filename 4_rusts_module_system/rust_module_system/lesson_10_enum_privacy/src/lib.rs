#![allow(dead_code)]

pub mod back_of_restaurant {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let _order1 = back_of_restaurant::Appetizer::Soup;
    let _order2 = back_of_restaurant::Appetizer::Salad;
}
