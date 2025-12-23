# Lesson 11: The `use` keyword and `pub use`

//^ Lets look at another example so we can talk about the use keyword

```rust
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
```

//. Specifying the full path when we call these functions is not very pretty or ideal.
//. So to get around that Rust provides the `use` keyword.
//. `use` allows you to bring a path into scope.
//. So as an example lets bring the hosting module into scope.

```rust
mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_restaurant::hosting; //. Now that our hosting module is in scope we can use it directly without specifying front_of_restaurant
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

//. The above code uses an absolute path for `use`.
//. We can also use a relative path.

```rust
mod front_of_restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_restaurant::hosting; //. Here self is referencing the current module
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

//. What if we want code from another file to also have access to the add_to_waitlist function.
//. Right now code in another file will only have access to the eat_at_restaurant function since its public.
//. To bring the hosting module into scope for external code we can add pub before the use keyword.
//. Like this:

```rust
pub use crate::front_of_restaurant::hosting; //. Now external code can also access functions in the hosting module
```
