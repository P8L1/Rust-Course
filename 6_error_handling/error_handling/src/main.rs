#![allow(warnings)] //. Stops the compiler from generating warnings
#![allow(errors)]

use std::io::Read;

fn main() {
    // ^ The panic macro
    //. If your program fails in a way that is unrecoverable or you cant handle the error gracefully then you can call the panic macro which will immediately quit your program, and print out an error message
    //. Example
   // panic!("crash and burn");  //. Running this program outputs:

    //.  thread 'main' panicked at src\main.rs:7:5:
    //. crash and burn
    //. note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    //. What is a backtrace?
    //. If I set the env variable RUST_BACKTRACE=1 then rust will print out all the function calls leading up to the panic. This wont be useful in our scenario

    //. Lets move on to an example that would benefit from a backtrace.
    a();

    fn a() {
        b();
    }
    fn b() {
        c(22);
    }
    fn c(num: i32) {
        if num == 22 {
            panic!("Don't pass in 22")
        }
    }

    //. Now running the program outputs
    //. thread 'main' panicked at src\main.rs:27:13:
    //.  Don't pass in 22

    //. Now this error does not really help us understand who is making the call to c (Remember in large codebases multiple functions can call one function)
    //. So lets set the env variable RUST_BACKTRACE=1 and run our program
    //. We can do it via the command "RUST_BACKTRACE=1 cargo run" if on linux or if on WIndows "$env:RUST_BACKTRACE="1"; cargo run"

    //. Now running the program outputs

    //     thread 'main' panicked at src\main.rs:27:13:
    //     Don't pass in 22
    //     stack backtrace:
    //    0: std::panicking::begin_panic_handler
    //              at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:697
    //    1: core::panicking::panic_fmt
    //              at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\panicking.rs:75
    //    2: error_handling::main::c
    //              at .\src\main.rs:27
    //    3: error_handling::main::b
    //              at .\src\main.rs:23
    //    4: error_handling::main::a
    //              at .\src\main.rs:20
    //    5: error_handling::main
    //              at .\src\main.rs:17

    //. So now its clear what functions are calling what (the lower the number the further down the call cain, the chain starts with the main function being called at 5 then main calls a then a calls..)

    //^ The Result enum

    //. The result enum is an enum value similar to the option enum. It is in your programs scope by default.
    //. Instead of having the value/none variants it has the value/error values
    //. The result enum is used for cases like opening a file.
    //. Opening a file can throw many errors, like the file could not exist or the file could be corrupt/wrong format ect..

    //. The Result enum looks like this (manual definition)
    enum Result<T, E> { //. T is a type. It can be any type (int string ect..) E is an error
        Ok(T),
        Err(E)
    }

    //^ Using Result

    //^ Example

    use std::fs::File; //. We bring the file struct into scope from std
    let f = File::open("hello.txt"); //. We attempt to open the file. The result of this attempt is stored in the variable file
    //. If it works the file is stored in the file variable. If it throws an error it is also stored in the file variable

    //. So now we need to handle the result case and the error case and to do that we can use a match expression

    let foo = match f {
                                Ok(file) => file, //. If the result of f is Ok (not an error) we give foo the value of file (The file we unwrapped)
                                Err(error) => panic!("There was a problem opening the file {:?}", error), //. If the result of f is an error then the program panics
                            };

    //. Now lets say we want to handle this more gracefully, instead of panicking when the file does not exist we want to just create a new file
    
    use std::io::ErrorKind; //. Brings into scope the error kind enum. It lets us match on the type of error we get

    let f = File::open("hello.txt");
    let foo = match f {
                                Ok(file) => file,  //. If f is a file (not an error) we bind its value to foo
                                Err(error) => match error.kind() { //. If f is not a file but an error we match the error on its kind
                                                                    ErrorKind::NotFound => match File::create("hello.txt"){ //. If the error kind it NotFound then we attempt to create the file. But creating the file returns a result enum
                                                                                                                Ok(fc) => fc, //. If the file was created successfully we give the value of the newly created file to foo
                                                                                                                Err(e) => panic!("Problem creating file: {:?}", e), //. If we encountered an error while creating the file then we panic 
                                                                                                            } 
                                                                    other_error => { //. If the original error we got when we tried to open the file is not of the type ErrorKind::NotFound bind the error to the other_error variable and then panic
                                                                        panic!("Problem opening file: {:?}", other_error)
                                                                    }
                                }
                            };
    //. The above code is hard to read, there is a better way to write it using closures. We will be learning about closures later on 
    
    //^ Unwrapping
    //. Here we are back to our original example where we are attempting to open the file then using a match expression to either get the file or panic if a error occurred
    let f = File::open("hello.txt"); 
    let foo = match f {
                                Ok(file) => file, 
                                Err(error) => panic!("There was a problem opening the file {:?}", error), 
                            };
    //. Instead of writing that match expression we can simplify our code by calling unwrap
    let f = File::open("hello.txt").unwrap(); //., Here unwrap does the same as our match statement. If the file was opened successfully then we take its value and give it to f (not foo this time). In the error case it will automatically panic
    //. You can also use .expect to specify the error that gets passed to the panic macro
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); //. Here the message will be failed to open hello.txt. (Expect does the same as unwrap but it also allows you to specify the error)

}

mod Error_propagation {
    //^ Error propagation
    //. Often times when you have a function thats implementation (code) calls something that can fail you want to return that error to the caller instead of handling it inline
    //. This gives more control to the caller who can decide what to do with that error. This is called error propagation

    //^ Example
    //. We have a function called read_username_from_file and the function returns a result type which could either be a string or a username or a error
    //. Inside our function we attempt to open hello.txt which could fail so open returns a result type which we store in f
    //. Next we have a match expression. If opening the file succeeds then we take that file and store it in f. If it throws an error then we take that error and return it
    //. Next we create a new string and then call read to string on our file. Which will read the contents of our file and store it in our string. Read to string returns a result type. So in the success case we return the string and in the error case we return an error
    //. Ignore the errors
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("Hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

mod the_question_mark_operator {

    //. We can simplify the above code further using Rusts built in features
    //. We can remove the first match and just put a question mark after open
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("Hello.txt")?; //. This will do something very similar to calling the .unwrap or .expect methods. If we succeed at opening the file then the file is returned and stored in f .
                                                                                    //. If we fail to get  the file then our function will just return the error (end early)

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

}

mod the_question_mark_operator_v2 {

    //. We can simplify the above code further by also deleting the second match and replacing it with a ?
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("Hello.txt")?; //. This will do something very similar to calling the .unwrap or .expect methods. If we succeed at opening the file then the file is returned and stored in f .
                                                                                    //. If we fail to get  the file then our function will just return the error (end early)
        let mut s = String::new();
        f.read_to_string(&mut s)?; //. If read_to_string works then we store it in s. If not we return the error.
        Ok(s) //. If this line is reached we know read_to_string worked and did not return an error. Thus s has a value that we just return
    }

}
mod the_question_mark_operator_v3 {
         //. In v2 we already simplified the function a lot but we can simplify it more by chaining method calls
        use std::io;
        use std::io::Read;
        use std::fs::File;

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("Hello.txt")?.read_to_string(&mut s)?; //. Here we chain methods using the ? operator
            Ok(s) 
        }
}

//. Lets simplify it more
mod the_question_mark_operator_v4 {
        use std::io;
        use std::fs::{self, File};

        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")  //. fs has a built in read_to_string function which returns an error or a file as a string
        }
}

// ^ Using ? in main.
//. Up to this point the main function always returned nothing. Thats valid but we can also make main return a result type.
//. So main can either return nothing or return a result type

//^ Example

mod main_example{
    use std::error::Error;
    use std::fs::File;

    //. In the code below we changed main to return a result type  Result<(), Box<dyn Error>>
    //. In the success case we return a unit type () which is basically nothing
    //. In the error case we return this weird thing Box<dyn Error>> which is a trait object and it basically means any type of error. We will learn more about trait objects later on.

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?; //. We can now use the question mark operator in our main function
        Ok(()) //. If everything succeeds then return the unit type ()
    }
}

//^ Now that we have covered the panic macro and the result enum the question stands, when should you be using either of these.

//. Best practice is to ALWAYS use the result enum along with error propagation. This prevents your program from crashing and error propagation allows the function caller to decide how to handle the error.
//. You should only ever use panic in EXCEPTIONAL circumstances, Circumstances in which recovering from the error is not possible and your program must end because its in a bad state
//. Another place to use the panic macro along with unwrap or expect is in example code to illustrate a concept
//. My rule of thumb: If it is going to be used by a user in ANY WAY then it needs to have proper error handling

//^ Type driven development
//. Type driven development is CRUCIAL to real world applications. Basically you want as much of it in your program as possible
//. Type Driven development is when you tie validation to a variables type. So any value of a specific type is GUARANTEED to be valid

//. I highly recommend you watch https://www.youtube.com/watch?v=NDIU1GSBrVI

//^ Here is a basic example of Type driven development


mod password_demo {
    //^ Big idea: "Password is a type"
    //. If you have a Password, it is guaranteed long enough.
    //. That means other code does NOT need to re-check length.

    pub const MIN_LEN: usize = 8; //. Every variable that is of the type passwords needs to have MIN_LEN

    pub struct Password {
        //. Private field: nobody outside this module can create Password directly.
        //. They MUST call Password::new(), which enforces the rule.
        value: String,
    }

    impl Password {
        pub fn new(raw: String) -> Option<Self> {
            //. The only rule: long enough.
            if raw.len() < MIN_LEN {
                return None;
            }
            Some(Self { value: raw })
        }

        pub fn as_str(&self) -> &str {
            &self.value
        }
    }
}

//. Now when we create a new user, that password has to be created by the new function, that will always check and make sure the password is long enough

//. Example usage
    // let password = match password_demo::Password::new(input) {
    //     Some(p) => p,
    //     None => {
    //         println!("Password too short.");
    //         return;
    //     }
    // };

//. Now we do it this way because now we are guaranteed that everything of the type password has a certain len. If we did not do it this way and there was 10 ways for users to create a password we would need to check that the password is valid 10 diffrent times
//. On large codebases this becomes a problem, since you may implement another way for users to create a password 6 months later but then forget to validate the length. TDD eliminates this











