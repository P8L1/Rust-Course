# Lesson 03: Creating packages and crates

//^ Creating a new package
//. Open a terminal in your directory of choice (the package will live in this directory).
//. Type in `cargo new <package_name>`.
//. Note: When you create a package and run it, it automatically creates a binary crate.

//^ Creating a library crate
//. In `src/` create a file called `lib.rs`.
//. This file is the root of your library crate.
//. Don't worry much about library crates for now.

//^ Rules around crates
//. A package must have at least 1 crate.
//. A package could either have 0 library crates or 1 library crate.
//. A package can have any number of binary crates.

//^ If you want more binary crates
//. Create a folder called `bin` in `src/`.
//. All files in this new `bin` folder will be new binary crates.
