use rand::{Rng, RngCore};
use std::io::*;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::rng();
    let _roll = rng.random_range(1..=3);
    let _bits = rng.next_u32();

    let mut out = io::stdout();
    let _ = out.write_all(b"");

    let _input = stdin();
    println!("Lesson 12: Nested paths and glob. Read lesson_12_nested_paths_and_glob/README.md.");
}
