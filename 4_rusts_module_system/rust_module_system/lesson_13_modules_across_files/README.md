# Lesson 13: Modules across multiple files

//^ Coding across multiple files
//. Lets make a new file called `front_of_restaurant.rs` and then define the front_of_restaurant module inside of it.
//. Now that I moved front_of_restaurant to the front_of_restaurant file we can define the module like this:

```rust
mod front_of_restaurant; //. This tells Rust "Define the front_of_restaurant module here but get the contents from another file front_of_restaurant.rs"
```

//. Everything in `front_of_restaurant.rs` is automatically inside the front_of_restaurant module.

```rust
//. Everything in here is automatically in the front_of_restaurant module

pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

//. Lets define hosting in another file.
//. We will start by making a new folder called `front_of_restaurant` in `src/` then in it we will make `hosting.rs`.
//. Then we paste the contents of the hosting module into this file.

```rust
//. Now we can just define hosting as

pub mod hosting;
```

//. The final file layout looks like this:

```text
src/
  lib.rs
  front_of_restaurant.rs
  front_of_restaurant/
    hosting.rs
```
