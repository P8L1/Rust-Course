#![allow(warnings)] //^ Rust wont show warnings


fn main() {


    //^ ----- Ownership rules -----
    //~ 1. Each value in Rust has a variable that's called its owner.
    //~ 2. There can only be one owner at a time.
    //~ 3. When the owner goes out of scope, the value will be dropped.
    //^ ----- Borrowing rules -----
    //~ 1. At any given time, you can have either one mutable reference or any number of immutable references.
    //~ 2. References must always be valid.


    //^ ----- Ownership rule 1 -----
    {
        //. s is not valid here, it's not yet declared
        let s = "hello"; //. s is valid from this point forward.
        //. do stuff with s
    } //. The scopre is now over, and s is no longer valid.

    //^ ----- Ownership rule 2 -----
    let x = 5;
    let y = x; //. Copy

    //$ What it looks like in memory. (The heap is not used)
    //@             Stack                                                  
    //.         ----------------
    //&             x : 5
    //&             y : 5
    //.         ----------------


    let string_one = String::from("Rusty"); //. Refer to (#####)
    let string_two  = string_one; //. Moves string_one into string two Refer to ($$$)
    println!("String one: {string_one}");  //. string_one does not exist anymore, it has been dropped
    
    let string_one = String::from("Rusty"); //. Refer to (#####)
    let string_two  = string_one.clone(); //. Clones string_one into string two Refer to (XYZ)
    println!("String one: {string_one}"); //. Works fine


    //$ What it looks like in memory. Note: ptr means pointer  (#####)  |
    //$                                                                 |
    //@         Stack                                                   |                                                    
    //@  _______________                      Heap                      |        
    //@ |  string_one  |____             _______________                |
    //& | name     | value  |           | index | value |               |
    //^ | ptr      |        | ----->    |  0    |   R   |               |
    //^ | len      |    5   |           |  1    |   u   |               |
    //^ | capacity |    5   |           |  2    |   s   |               |
    //^                                 |  3    |   t   |               |
    //^                                 |  4    |   y   |               |
    //$                                                                 |
    //$ ----- END OF (#####) -----                                      |                                    

    //. ----- START OF ($$$) -----                                      |
    //@          Stack                                                  |
    //@  _______________                      Heap                      |
    //@ |  string_two  |____             _______________                |
    //& | name     | value  |           | index | value |               |
    //^ | ptr      |        | ----->    |  0    |   R   |               |
    //^ | len      |    5   |           |  1    |   u   |               |
    //^ | capacity |    5   |           |  2    |   s   |               |
    //^                                 |  3    |   t   |               |
    //^                                 |  4    |   y   |               |
    //.                                                                 |
    //. Note string one does not exist anymore                          |
    //. ----- END OF ($$$) -----                                        |
                                    

    //. ----- START OF (XYZ) -----                                      |
    //@          Stack                                                  |
    //@  _______________                      Heap                      |
    //@ |  string_one  |____             _______________                |
    //& | name     | value  |           | index | value |               |
    //^ | ptr      |        | ----->    |  0    |   R   |               |
    //^ | len      |    5   |           |  1    |   u   |               |
    //^ | capacity |    5   |           |  2    |   s   |               |
    //^                                 |  3    |   t   |               |
    //^                                 |  4    |   y   |               |
    //.                                                                 |
    //@          Stack                                                  |
    //@  _______________                      Heap                      |
    //@ |  string_two  |____             _______________                |
    //& | name     | value  |           | index | value |               |
    //^ | ptr      |        | ----->    |  0    |   R   |               |
    //^ | len      |    5   |           |  1    |   u   |               |
    //^ | capacity |    5   |           |  2    |   s   |               |
    //^                                 |  3    |   t   |               |
    //^                                 |  4    |   y   |               |
    //.                                                                 |
    //.                                                                 |
    //. ----- END OF (XYZ) -----                                        |

    let s = String::from("hello");
    takes_ownership(s); //. The function takes_ownership is now the owner of s and it is thus dropped in MAINS scope
    println!("The value of the string is {}", s); //. This wont work since s has been dropped

    let x = 10;
    copies_ownership(x); //. Integers are copied NOT MOVED
    println!("The value of the x is {}", x); //. Thus x is still valid here

    //% ----- REFRENCES -----
    //% Introduction: Not being able to use a value after calling a function can be problematic if you happen to need to use the value after the function call

    //^ EXEMPLE ----------------
    let s = String::from("hello");
    takes_ownership(s);
    let first_letter = get_first_letter_example(s); //^ This is a problem since I want to still be able to use s (in the current function) after calling a function with s
    //^ ----------------

    //& FIX using a REFRENCE
    let s_one = String::from("hello");
    let refrence_to_s_one = &s_one; //^ Creates a refrence to s_one and stores it in the variable refrence_to_s_one
    let first_letter = get_first_letter_fix(refrence_to_s_one); //^ Call the function get_first_letter_fix with a REFRENCE to s_one. This keeps ownership of s_one in the current scope
    println!("The value of s is {}", s_one);  //. s is still valid here since we did not transfer ownership
    println!("The first letter of s is {}", first_letter); //. Will print h

    //^ What a refrence looks like in memory

    //@          Stack                    Stack                                                 |
    //@  ___________________     ___________________                   Heap                     |
    //@ | refrence_to_s_one |   |        s_one      |            _______________                |
    //& | name     | value  |   | name     | value  |           | index | value |               |
    //^ | ptr      |        | ->| ptr      |        | ----->    |  0    |   R   |               |
    //^                         | len      |    5   |           |  1    |   u   |               |
    //^                         | capacity |    5   |           |  2    |   s   |               |
    //^                                                         |  3    |   t   |               |
    //^                                                         |  4    |   y   |               |
    
    //. Passing in refrences as function parameters is called borrowing because we are borrowing the value but we are not actually taking ownership.
    //. NOTE: Refrences are immutable by default. Here is an example of a mutable refrence

    let mut s_one = String::from("foo"); //. The variable being refrenced needs to be mutable
    let mut refrence_to_s_one = &mut s_one; //. Make a mutable refrence
    change_s_one(refrence_to_s_one);
    println!("The value of s_one is {}", s_one); //. Will print "foobar"

    //^ Mutable refrences do have a big restriction. You can only have one mutable refrence to a piece of data in one scope
    //^ This is borrowing Rule 1 -----------------

    //^ Example 1 --> You can only have 1 mutable refrence to a piece of data
    {
        let mut s = String::from("Hello");
        let r1 = &mut s; //. r1 is a mutable refrence to s
        let r2 = &mut s; //. This cant happen since we already have a mutable refrence to s and cant have 2 at the same time
        println!("The value of r1 is {}, the value of r2 is {}", r1, r2)
    }

    //^ Example 2 --> You can have any number of immutable refrences
    {
        let s = String::from("Hello");
        let r1 = &s; //. r1 is a refrence to s
        let r2 = &s; //. r2 is a refrence to s
        println!("The value of r1 is {}, the value of r2 is {}", r1, r2)
    }

    //^ Example 3 --> You cant have a mutable refrence if an immutable refrence already exists
    {
        let mut s = String::from("Hello");
        let r1 = &s; //. r1 is a refrence to s
        let r2 = &mut s; //. This wont work since we already have an immutable refrence to s. Immutable refrences dont expect the value of the variable they are refrencing to change. So r1 does not expect the value of s to change. Thus we cant create a mutable refrence to s since that allows s to change.
        println!("The value of r1 is {}, the value of r2 is {}", r1, r2)
    }

    //& IMPORTANT:  Note: The scope of a refrence starts when its first introduced and ends when its used for the last time

    //^ Example 4 --> How to mix mutable and immutable refrences
    {
        let mut s = String::from("Hello");
        let r1 = &s; //. r1 is a refrence to s ------------------------------------------------------------------------------ r1 scope starts
        println!("The value of r1 is {}", r1); //. r1 is used for the last time here. Thus its scope ends here and it is dropped. ------ r1 scope ends
        //. r1 is no longer present in memory here since it has been dropped on the last use (it was last used on the previous line and has thus already been dropped out of memory)
        //. Since r1 does not exist, there are no immutable refrences to s. Thus we are allowed to create a mutable refrence.
        let r5 = &mut s; //. Valid since no immutable refrences to s exist
        r5.push_str(", there!");
        println!("{}", s) //. Will print "Hello, there!"
    }

    //^ What happens when we have refrences to invalid data?

    //^ Example 1
    {
        fn main() {
            let refrence_to_nothing = dangle(); //. The result of dangle() is a refrence to a piece of data that does not exist
        }
        fn dangle() -> &String  {
            let s = String::from("Hello");
            return &s;
        } //. s goes out of scope here and is thus dropped. So the string &s is refrencing does not exist. Thus we get the error "this function's return type contains a borrowed value, but there is no value for it to be borrowed from"
        //. This can be fixed with lifetimes. You will learn about lifetimes in the far future
    }




    //^ Slices ----------------------------
    //^ Slices let you refrence a continues sequence of elements within a collection instead of refrencing the entire collection
    //^ Just like the refrences we covered above slices do not take ownership of the underlying data.

    //^ To understand why slices are useful lets start with a problem

    //^ Problem 1

    //. Lets say we have a function to return the first word in a string what would our return type be 
    //. We could return an index to the first space we find? For emaple if the String is "Hi how" then we return 2 since the first space is at index 2. Thus everything before index 2 contains the first word.

    //~ The implimentation would look something like this
    {
        fn main() {
            let mut s = String::from("Hi, how are you?");
            let word = first_word(&s); //. value of word is 3
            s.clear();
        }
        fn first_word(some_string: &String) -> usize {
            let some_string_as_bytes = some_string.as_bytes(); //. Convert string to an array of bytes
            for (i, &item) in some_string_as_bytes.iter().enumerate() {
                if item == b' ' { //. Check if the current item is a space
                    return i;   //. If the current item is a space then we return the current ietems index
                }
            }
            return some_string.len(); //. If no spaces we found then we return the len of the string.
        }
    }

    //^ There are two problems with this implimentation

    //^ The first being that the return value is not tied to the string.
    //^ Heres what I mean 

    {
        fn main() {
            let mut s = String::from("Hi, how are you?");
            let mut word = first_word(&s); //. value of word is 3
            s.clear(); //^ This sets the string to an empty string aka ""
            //^ The string s is now empty but the value of word is still 3. But there are no words in s since it has been cleared. So thus word having a value of 3 is not accurate
            //^ This means we need to call the first word function again like this everytime we update the value of s
            word = first_word(&s)
            //^ THIS IS BAD CODE AND A RECIPIE FOR BUGS
        }
        fn first_word(some_string: &String) -> usize {
            let some_string_as_bytes = some_string.as_bytes(); //. Convert string to an array of bytes
            for (i, &item) in some_string_as_bytes.iter().enumerate() {
                if item == b' ' { //. Check if the current item is a space
                    return i;   //. If the current item is a space then we return the current ietems index
                }
            }
            return some_string.len(); //. If no spaces we found then we return the len of the string.
        }
    }

    //^ The second problem is that the implimentation is extrmemely hard to refactor. What if we wanted to return the second word? Then we would need to return a tuple with the start of the word and the end of the word.

    //^ To get around these issues lets introduce the string slice!

    //^ EXMPLE OF A STRING SLICE
    let mut some_string = String::from("Hello world");
    let first_word = &some_string[..5]; //. Ask rust to give us a refrence to the first 5 char of the string called some_string                     NOTE: I wrote [..5] instead of [0..5] This is because if you are starting at the start of a string you dont have to define a starting point. This is optional and no behaviour would change if you wrote [0..5] instead
    let second_word = &some_string[6..]; //. Ask rust to give us a refrence to characters 6 to the last char of the string called some_string       NOTE: I wrote [6..] instead of [6..11] This is because if you want your string slice to end at the end of a string then you dont have to define an ending char. This is optional and no behaviour would change if you wrote [6..11] instead
    //. NOTE IF YOU WANT A STRING SLICE TO SPAN THE ENTIRETY OF A STRING JUST USE [..] eg &some_string[..]


    //~ The implimentation of first_word using string slices would look something like this
        {
        fn main() {
            let mut some_string = "Hello world";
            let word = first_word(some_string);
            println!("The value of word is: {}", word); //^ WIll print Hello
            some_string = "abc"; 
            println!("The value of word is: {}", word); //^ Will print abc
            //^ The value of word is thus tied to the string some_string. If it changes the value of word also changes
        }
        fn first_word(some_string_input: &str) -> &str { // Returns a string slice
            let some_string_as_bytes = some_string_input.as_bytes(); //. Convert string to an array of bytes
            for (i, &item) in some_string_as_bytes.iter().enumerate() {
                if item == b' ' { //. Check if the current item is a space
                    return &some_string_input[0..i];  //^ Return a string slice from the start of the word to the index of the first space
                }
            }
            return &some_string_input[..]; //^ If no spaces we found then we return the entire string as a string slice
        }
    }

    //^ You can also make slices on other data types

    //^ Example one -> Integer array
    {
        let andre_se_punte = [12, 15, 31, 50, 4, 17];
        let wiskunde_en_fisika = &andre_se_punte[..1];

    }
}

fn takes_ownership(s: String) {
    println!("The value of the string is {}", s);
} //. s is dropped out of the programs scope

fn copies_ownership(x: i32) {
    println!("The value of the x is {}", x);
}

fn get_first_letter_example(s: String) -> char {
    return s.chars().nth(0).expect("String has no chars");
}

fn get_first_letter_fix(s: &String) -> char { //. You need to declare the function will take a refrence to a String
    return s.chars().nth(0).expect("String has no chars");
}  //. The value of s is dropped

fn change_s_one(some_string: &mut String) { //. You need to declare the function will take a mutable refrence to a String
    return some_string.push_str("bar");
}