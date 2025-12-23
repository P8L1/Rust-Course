# Lesson 04: Defining modules and a module tree

//^ Defining your own modules
//. Lets create a new package that contains a library crate.
//. Lets call this package restaurant.
//. Run `cargo new --lib restaurant`.
//. Now in the `src` folder you have `lib.rs` instead of `main.rs`.

//. Our goal is to make a library that helps a restaurant.
//. We will think of it in two parts.
//. 1. The front of the restaurant that serves customers.
//. 2. The back of the restaurant that prepares the food, etc.

//. Paste this code in `lib.rs`.

//. Start Paste ========================

```rust
mod front_of_restaurant {   //. A module is defined with the mod keyword followed by the name of the module and then curly brackets
//.                             DO NOT GET CONFUSED mod here does not mean mod as in divide and take the remainder. That mod is % e.g. 6 % 5 = 1. This mod means module
    
    //. As you can see modules can contain other modules inside of them, they can also contain structs, functions, enums, crates etc.
    //. Structuring code this way keeps it organised.
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
```

//. End Paste ========================

//. For the above code our module tree looks like this

```text
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
```
