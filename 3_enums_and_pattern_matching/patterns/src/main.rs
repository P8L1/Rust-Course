fn main() {
    let x = '5';
    let y = x.to_digit(10);

    let z = y + 5;
}

enum Optional<T> {
    Some(T),
    None,
}

enum Optional {
    Some(u32),
    None,
}