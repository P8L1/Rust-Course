# Lesson 10: Enum privacy

//^ Lets look at another example with enums
//. Start Paste ========================

```rust
mod back_of_restaurant {
    enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_restaurant::Appetizer::Soup;
    let order2 = back_of_restaurant::Appetizer::Salad;
    //. The above code gives an error since just like structs enums are private by default so we thus need to make the enum public.
}
```

//. End Paste ========================

//. Start Paste ========================

```rust
mod back_of_restaurant {
    pub enum Appetizer { //. Made it public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_restaurant::Appetizer::Soup;
    let order2 = back_of_restaurant::Appetizer::Salad;
    //. Notice how the variants of the enum are automatically marked public if the enum is marked public.
}
```

//. End Paste ========================
