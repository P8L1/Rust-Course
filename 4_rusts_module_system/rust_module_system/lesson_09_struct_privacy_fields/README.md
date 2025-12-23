# Lesson 09: Struct privacy and fields

//^ Privacy rules when it comes to structs
//. Paste this code (delete the previous).
//. Start Paste ========================

```rust
mod back_of_restaurant {
    struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

//. Now lets define a function to eat at the restaurant

pub fn eat_at_restaurant() {
    let mut meal = back_of_restaurant::Breakfast::summer("toast1");
    //. Now we have the errors. Our Breakfast struct is private by default (so we cant access it).
    //. Also our summer associated function is also private by default.
    //. To fix these errors lets make them public.
}
```

//. End Paste ========================

//. Now the code looks something like this.
//. Remove the old code and use this new code.
//. Start Paste ========================

```rust
mod back_of_restaurant {
    pub struct Breakfast { //. Added pub
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { //. Added pub
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_restaurant::Breakfast::summer("toast1");
    //. Now that those errors are gone lets change the toast attribute of meal to be set to toast2.
    meal.toast = String::from("toast2");
    //. Now you get the error "the toast of struct Breakfast is private".
    //. This is because all fields in a struct by default are private.
    //. To fix this we need to mark toast as a public field.

}
```

//. End Paste ========================

//. Now to make toast public we can use this code.
//. Start Paste ========================

```rust
mod back_of_restaurant {
    pub struct Breakfast {
        pub toast: String,  //. Added pub
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
    meal.toast = String::from("toast2"); //. We can now successfully reassign.
}
```

//. End Paste ========================
