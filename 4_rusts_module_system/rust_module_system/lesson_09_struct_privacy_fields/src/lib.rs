#![allow(dead_code)]

pub mod back_of_restaurant {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_restaurant::Breakfast::summer("toast1");
    meal.toast = String::from("toast2");
    let _ = meal;
}
