# Lesson 12: Nested paths and the glob operator

//^ Nested paths
//. Lets say I added the rand dependency to Cargo.toml and I want to bring some of its functions into scope.
//. I could include them like this:

```rust
use rand::Rng;
use rand::RngCore;
```

//. But since both instances of use above start with rand we can simplify it like this:

```rust
use rand::{Rng, RngCore};
```

//. Another example:

```rust
use std::io;
use std::io::Write;
```

//. Both of the above lines include the std::io common path so we can simplify it using:

```rust
use std::io::{self, Write}; //. Here self refers to the actual io module
```

//^ The glob operator
//. Lets say we wanted to bring all the public items in io into scope.
//. To do this we can use:

```rust
use std::io::*; //. Now all public items in the io module are automatically in scope
```
