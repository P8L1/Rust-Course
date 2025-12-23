#![allow(warnings)] //. Stops the compiler from generating warnings


//~ Up to this point we have always been writing all of our code in one file which lives in the default module.
//~ For any project this will not cufise. Imagine how long your IDE would take to load a file with 100K+ lines of code. Imagine how hard it would be to find specific code within your project.
//~ Imagine the security risks it would impose

//~  To solve these problems in the real word we split projects across lots of files. My personal rule of tumb is to keep all your files at under 200 lines of code where possible. Also split files by feature.

//~ Lets take a look at another problem; you are in charge of writing a Authentication service for a backend. You dont want any other parts of the codebase to have acess to the source code. 
//~ If other parts of the codebase have acess to your authentication code then it would be easy for other employees at the compony to leak how Authentication at the compony works. This poses a security risk

//~ So we also sometimes want to write code that other parts of the project can use but cant neccesarily read. This is called encaptulation


//. Rust has a module system that starts with a Package
//. When you type in cargo new you are creating a new package
//. A package stores crates
//. A crate can either be a binary crate (code that is already in the form of 1's and 0's ready to be executed) or a library crate which is code that can be used by other programs 
//. Crates contain modules. modules allow you to organise chunks of code and control the privacy rules. So going back to the authentication example, lets say you have a library crate that contains an authentication module. You can then make the code inside your authentication module private but for example expose one public login method
//. If we wanted code outside this library to call the login method then it would need to specify the path to the login method.

//^ Creating a new Package
//. Open a terminal in your directory of choice (The package will live in this directory)
//. Type in "cargo new <packagename>"
//. Note: When you create a package and run it, it automaticlly creates a binary crate

//^ Creating a library crate
//. In src/ create a file called lib.rs
//. This file is the root of your library crate 
//. Dont worry much about library crates for now


//^ Rules around crates
//. A package must have atleast 1 crate 
//. A package could either have 0 library crates or 1 library crate
//. A package can have any number of binary crates

//^ If you want more binary creates
//. Create a folder called bin in src/ 
//. All files in this new bin folder will be new binary crates

//^ Defing your own modules 
//. Lets create a new Package that contains a library crate
//. Lets call this package restaurant
//. Run cargo new --lib restaurant
//. Now in the src folder you have lib.rs instead of main.rs

//. Our goal is to make a library that helps a restaurant
//. We will think of it in two parts
//. 1. The front of the restaurant that serves customers
//. 2. The back of the restaurant that prepares the food ect..

//. Paste this code in lib.rs

//. Start Paste ========================

mod front_of_restaurant {   //. A module is defined with the mod keyword followed by the name of the module and then curly brackets
//.                             DO NOT GET CONFUSED mod here does not mean mod as in divide and take the remainder. That mod is % eg. 6 % 5 = 1. This mod means module
    
    //. As you can see modules can contain other modules inside of them, they can also contain structs, functions, enums, crates ect..
    //. Structuring code this way keeps it organised
    //. If in the future we wanted to add a function to seat people at a VIP table we would know exactly where to put that function.

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

//. For the above code our module tree looks like this

//. crate
//.    |
//.    | ---- front_of_restaurant 
//.                    |
//.                    | ----- hosting
//.                    |           |
//.                    |           |---- add_to_waitlist
//.                    |           |
//.                    |           |---- seat_at_table
//.                    |
//.                    | ----- serving
//.                                |
//.                                |---- take_order
//.                                |
//.                                |---- serve_order
//.                                |
//.                                |---- take_payment
//. End Paste ========================


//^ Lets look at a simpiler example first
//. Delete your entire lib.rs and paste this in

//. Start Paste ========================

mod front_of_restaurant {
    mod hosting { //. Hosting is the child of front_of_restaurant since it is defined inside of front_of_restaurant
        fn add_to_waitlist() {} //. the add_to_waitlist function is the child of hosting
    }
}

//. Now lets try calling the add_to_waitlist function
//. To do that we need to tell rust the exact path it needs to take to reach the function
pub fn eat_at_restaurant() {
    //. Absolute path. This is the full path from the root (from the start)
    crate::front_of_restaurant::hosting::add_to_waitlist();
    //. Relative path. Since we are already in the lib.rs folder we dont need to specify crate we can just use
    front_of_restaurant::hosting::add_to_waitlist();
    //. Relative paths start from the current module (We are already in crate:: so it can be ommited)
    //. Absolute paths start from the root (from crate)
}

//^ Module Privacy rules
//. Notice we have some errors in the above code
//. The error reads that the hosting module is private
//. This is because of Rust's privacy rules
//. The idea of privacy rules is for one function to not be visable to other functions. (One function cant be called by other functions, other functions dont know it exists)

//^ Rule:
//. By default a child module and everything inside of it is private from the perspective of the parent module
//. So in this case front_of_restaurant cant see anything defined in hosting (It cant even see hosting)
//. On the flip side child modules can see everything in their parent modules. So if I define a variable inside of front_of_restaurant I can read its value from hosting
//. But if I define a variable inside of hosting I cant read its value from front_of_restaurant

//. This system allows us to hide implimentation details by default and only expose certin methods

//^ How to expose a function or module

//. The pub keyword marks a specific module or function as public, making it visable to outside code
//. In the above case we want to expost the add_to_waitlist function so that we can call it from outside of the front_of_restaurant module
//. We can do this by adding pub infront of the mod hosting (this makes the module public)
//. Now we still need to make the specific add_to_waitlist function public we can do this by also inserting the pub keyword infront of the fn add_to_waitlist

//. The code now looks like this (Delete the previous code so it wont throw errors)

mod front_of_restaurant {
    pub mod hosting { 
        pub fn add_to_waitlist() {} 
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_restaurant::hosting::add_to_waitlist();
    front_of_restaurant::hosting::add_to_waitlist();
}

//. Now we are able to access the add_to_waitlist function from outside the front_of_restaurant module
//. End Paste ========================

//^ Lets look at another example using the "super" keyword

//. Paste this code
//. Start Paste ========================

fn serve_order() {

}

mod back_of_restaurant {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

//. Here super is just saying Go up one module level from where I am now, and then look for this item

//. End Paste ========================

//^ Privacy rules when it comes to structs
//. Paste this code (Delete the previous)
//. Start Paste ========================

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
    //. Now we have the errors. Our Breakfast struct is private by default (So we cant acess it)
    //. Also our summer assosiated function is also private by default
    //. to fix these errors lets make them public
}
//. End Paste ========================

//. Now the code looks something like this
//. Remove the old code and use this new code
//. Start Paste ========================
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
    //. Now that those errors are gone lets change the toast attribute of meal to be set to toast2
    meal.toast = String::from("Toast2");
    //. Now you get the error "the toast of struct breakfeast is private"
    //. This is because all fields in a struct by default are private.
    //. To fix this we need to mark toast as a public field

}
//. End Paste ========================

//. Now to make toast public we can use this code 
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
    meal.toast = String::from("Toast2"); //.  We can now succesfully reassign
}


//^ Lets look at another example with enums
//. Start Paste ========================
mod back_of_restaurant {
    enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_restaurant::Appetizer::Soup;
    let order2 = back_of_restaurant::Appetizer::Salad;
    //. The above code gives an error since just like structs enums are private by default so we thus need to make the enum public
}
//. End Paste ========================

//. Start Paste ========================
mod back_of_restaurant {
    pub enum Appetizer { //. Made it public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_restaurant::Appetizer::Soup;
    let order2 = back_of_restaurant::Appetizer::Salad;
   //. Notice how the variants of the enum are automaticlly marked public if the enum is marked public
}
//. End Paste ========================

// ^ Lets look at another example so we can talk about the user keyword

mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    front_of_restaurant::hosting::add_to_waitlist();
    front_of_restaurant::hosting::add_to_waitlist();
    front_of_restaurant::hosting::add_to_waitlist();
}

//. Specifying the full path when we call these functions is not very pretty or ideal.
//. So to get around that rust provides the use keyword 
//. Use allows you to bring a path into scope
//. So as an example lets bring the hosting module into scope


mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_restaurant::hosting; //. Now that our hosting module is in scope we can use it directly without specifying front_of_house
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//. The above code uses an absulute path for uses. 
//. We can also use a realitive path

mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_restaurant::hosting; //. Here self is refrencing the current module
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//. What if we want code from another file to also have acess to the add_to_waitlist function.
//. Right now code in another file will only have acess to the eat_at_restaurant function since its public
//. To bring the hosting module into scope for external code we can add pub before the use keyword
//. Like this 

pub use crate::front_of_restaurant::hosting; //. Now external code can also acess functions in the hosting module

//^ Nested paths
//. Lets say I added the rand dependency to Cargo.toml and I want to bring some of its functions into scope

//. I could iclude them like this

use rand::Rng;
use rand::RngCore;

//. But since both instances of use above start with rand we can simplify it like this

use rand::{Rng, RngCore};

//. Another example

use std::io;
use std::io::Write;

//. Both of the above lines include the std::io comman path so we can simplify it using

use std::io::{self, Write}; //. Here self refers to the actual io module

//^ The glob operator 
//. Lets say we wanted to bring all the public ietems in io into scope
//. To do this we can use

use std::io::*; //. Now all public ietems in the io module are automaticcly in scope

//^ Coding across multiple files

//. Lets make a new file called front of restaurant and then define the front of restuarant function inside of it. 
//. Now That I moved front of restuarant to the front of restaurant file we can define the module like this

mod front_of_restaurant; //. This tells rust "Define the front_of_restaurant module here but get the contents from another file front_of_restaurant"