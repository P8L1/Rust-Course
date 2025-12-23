# Lesson 02: Packages, crates, modules, and paths

//. Rust has a module system that starts with a package.
//. When you type in `cargo new` you are creating a new package.
//. A package stores crates.
//. A crate can either be a binary crate (code that is already in the form of 1s and 0s ready to be executed) or a library crate which is code that can be used by other programs.
//. Crates contain modules. Modules allow you to organise chunks of code and control the privacy rules. So going back to the authentication example, lets say you have a library crate that contains an authentication module. You can then make the code inside your authentication module private but, for example, expose one public login method.
//. If we wanted code outside this library to call the login method then it would need to specify the path to the login method.
