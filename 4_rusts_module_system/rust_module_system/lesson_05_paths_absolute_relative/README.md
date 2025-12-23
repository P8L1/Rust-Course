# Lesson 05: Paths (absolute and relative)

//^ Lets look at a simpler example first
//. Delete your entire `lib.rs` and paste this in.

//. Start Paste ========================

```rust
mod front_of_restaurant {
    mod hosting { //. Hosting is the child of front_of_restaurant since it is defined inside of front_of_restaurant
        fn add_to_waitlist() {} //. The add_to_waitlist function is the child of hosting
    }
}

//. Now lets try calling the add_to_waitlist function.
//. To do that we need to tell Rust the exact path it needs to take to reach the function.
pub fn eat_at_restaurant() {
    //. Absolute path. This is the full path from the root (from the start).
    crate::front_of_restaurant::hosting::add_to_waitlist();
    //. Relative path. Since we are already in the lib.rs folder we dont need to specify crate we can just use
    front_of_restaurant::hosting::add_to_waitlist();
    //. Relative paths start from the current module (we are already in crate:: so it can be omitted).
    //. Absolute paths start from the root (from crate).
}
```

//. End Paste ========================

//. The `crate::` prefix always means “start from the root of this crate.”
//. The code above does NOT compile yet because of privacy rules; we will fix that in the next lessons.
//. For this crate to build, the real source code marks `hosting` and `add_to_waitlist` as public.
