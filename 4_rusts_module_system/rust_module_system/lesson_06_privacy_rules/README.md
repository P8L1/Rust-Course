# Lesson 06: Module privacy rules

//^ Module privacy rules
//. Notice we have some errors in the above code.
//. The error reads that the hosting module is private.
//. This is because of Rust's privacy rules.
//. The idea of privacy rules is for one function to not be visible to other functions. (One function cant be called by other functions, other functions dont know it exists.)

//^ Rule:
//. By default a child module and everything inside of it is private from the perspective of the parent module.
//. So in this case front_of_restaurant cant see anything defined in hosting (it cant even see hosting).
//. On the flip side child modules can see everything in their parent modules. So if I define a variable inside of front_of_restaurant I can read its value from hosting.
//. But if I define a variable inside of hosting I cant read its value from front_of_restaurant.

//. This system allows us to hide implementation details by default and only expose certain methods.

//. Broken example (this does not compile yet):

```rust
mod front_of_restaurant {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_restaurant::hosting::add_to_waitlist();
}
```

//. We will fix this in the next lesson using `pub`.
