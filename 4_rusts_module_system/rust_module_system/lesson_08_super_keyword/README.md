# Lesson 08: The `super` keyword

//^ Lets look at another example using the "super" keyword

//. Paste this code
//. Start Paste ========================

```rust
fn serve_order() {

}

mod back_of_restaurant {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

//. Here super is just saying go up one module level from where I am now, and then look for this item.

//. End Paste ========================
