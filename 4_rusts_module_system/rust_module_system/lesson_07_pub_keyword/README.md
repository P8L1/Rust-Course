# Lesson 07: Making items public with `pub`

//^ How to expose a function or module
//. The pub keyword marks a specific module or function as public, making it visible to outside code.
//. In the above case we want to expose the add_to_waitlist function so that we can call it from outside of the front_of_restaurant module.
//. We can do this by adding pub in front of the mod hosting (this makes the module public).
//. Now we still need to make the specific add_to_waitlist function public; we can do this by also inserting the pub keyword in front of fn add_to_waitlist.

//. The code now looks like this (delete the previous code so it wont throw errors):

```rust
mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_restaurant::hosting::add_to_waitlist();
    front_of_restaurant::hosting::add_to_waitlist();
}
```

//. Now we are able to access the add_to_waitlist function from outside the front_of_restaurant module.
//. End Paste ========================
