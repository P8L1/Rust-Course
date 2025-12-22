#![allow(warnings)] //. Stops the compiler from generating warnings



fn main() {

    //. Use enums if something is one of a set of diffrent options, lets use traffic lights as an example.
    //. The current active traffic light is one of (Red, Orange, Green). 

    //^ Use enums WHEREVER POSSIBLE. ENUMS PROMOTE GOOD CODE, AND PREVENT BUGS

    //^ Example one: IP Adresses, an IP can be one of (IPVersion4, IPVersion6). So for every single active IP adress it is either version 4 or version 6

    //^ Creating an enum
    enum IpAddrVersion { //. This creates an enum with the name IpAddrVersion 
        V4, //. This V4 is a variant of IpAddrVersion. It is one of the "one of" options from an IP can be one of (IPVersion4, IPVersion6)
        V6, //. Once again V6 is a variant of the enum IpAddrVersion
    }

    //^ Now lets create instances of IpAddrVersion

    let four = IpAddrVersion::V4; //. The syntax for creating an instance is <enum name>::<enum variant>. In this case the enum is named IpAddrVersion and the variant is called V4. Thus we use IpAddrVersion::V4
    let six = IpAddrVersion::V6;
    
    //. We now have a way to capture the version of an ip adress but what if we want to capture the IP adress itself
    //. For this we can use structs like we learned in the last chapter

    struct IpAddr {
        version: IpAddrVersion,
        address: String,
    }

    //^ Now lets create localhost it is an IpAddr
    let localhost = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("127.0.0.1"),
    };

    //^ The above code is still shit. (Good for Java but bad for rust)
    //^ Its to "all over the place", since we have the extra overhead and complexity of another struct

    //^ Lets improve it in this example by defining the address field in the enum itself.
    {
        enum IpAddrVersion { 
            V4(String), //. Now when something is of type IpAddrVersion::V4 it also has a string (for adress) so its more like IpAddrVersion::V4(String)
            V6(String), 
        }

        //^ Then we define localhost like this:
        let localhost = IpAddrVersion::V4(String::from("127.0.0.1"));
    }

    //^ Enum variants can store diffrent types of data, not just strings. Lets improve on our above code once again by storing a tuple of 4 ints instead of a string

    {
        enum IpAddrVersion { 
            V4(u8, u8, u8, u8), 
            V6(String), 
        }

        //. Now we can use it to create localhost like this
        let localhost = IpAddrVersion::V4(127, 0, 0, 1);

    }

    //^ Enum variants can store a wide variaty of types, to demonstate lets introducte the Message enum

    enum Message {
        Quit,                       //. Stores no Data
        Move { x : i32, y: i32},    //. Stores anonomys struct
        Write(String),              //. Stores a string
        ChangeColor(i32, i32, i32), //. Stores a integer tuple with a size of 3
    }

    //^ Just like structs we can define methods and associated functions on our enum type
    //. Example

    impl Message {
        fn some_function() {
            println!("Let's get Rusty");
        }
    }

    //^ Call it with
    Message::some_function();

    //^ The option enum

    //. Introduction: Some languages have a built-in value called "null"
    //. (for example: Dart, Java, Kotlin, etc.).
    //.
    //. "null" is used to mean: “there is no value here.”
    //. In other words, the value is missing / absent.
    //.
    //. Why would we ever want “no value”?
    //. Because sometimes something *might* exist, but it also might *not*.
    //. So we need a way to represent both possibilities.
    //.
    //. Example: converting a character into a digit
    //. - If the character is numeric like '5', we can convert it into the number 5.
    //. - If the character is not numeric like 'a', then there is no digit we can convert it to.
    //.   In many languages, that failure is represented using null.
    //.
    //. So you can think of null as: “nothing here” / “missing value”.
    //.
    //. The problem with null:
    //. Null is *very easy to forget about*.
    //. If your code expects a real value but accidentally gets null, your program can crash
    //. (often called a "null pointer exception").
    //.
    //. This was such a common and serious issue that Tony Hoare (who introduced the idea)
    //. later called it his “billion-dollar mistake.”
    //.
    //. Rust avoids null entirely.
    //. Instead, Rust uses an "Option" type (an enum) to represent “maybe a value, maybe not.”
    //.
    //. The key idea:
    //. If something might be missing, Rust forces you to handle that case explicitly.
    //. You can't accidentally “forget” about the missing case like you often can with null.
    

    //. This is what Rusts built in Option enum looks like
    //. enum Option<T> {
            //. Option is an "enum" (short for "enumeration").
            //. An enum is a type that can be ONE of several named variants.
            //. So an Option<T> value must be either:
            //.   - Some(T)
            //.   - None
            //. and it can NEVER be both at once.
        
            //. The <T> part means Option is "generic".
            //. "Generic" means: Option doesn't care what type it holds.
            //. T is a placeholder name for “whatever type you choose”.
            //.
            //. Examples:
            //.   Option<i32>    -> an optional i32 (maybe a number, maybe missing)
            //.   Option<char>   -> an optional char
            //.   Option<String> -> an optional String
            //.
            //. Think of it like a box:
            //.   Option<T> is a box that EITHER contains a T (Some(T)) OR is empty (None).
        
    //^      Some(T),
            //. Some is the variant that means: “A value IS present.”
            //. The (T) means: Some stores ONE value inside it, and that value has type T.
            //.
            //. Example with numbers:
            //.   let x: Option<i32> = Some(5);
            //.   - The type is Option<i32>
            //.   - The value is Some(5)
            //.   - Meaning: “x has a number, and it's 5”
            //.
            //. Example with characters:
            //.   let c: Option<char> = Some('a');
            //.   Meaning: “c has a char, and it's 'a'”
            //.
            //. Important: Some(...) is NOT “extra” or “wrapper for fun”.
            //. It is how Rust *labels* that a value exists.
        
    //^      None,
            //. None is the variant that means: “No value is present.”
            //. None does NOT store anything inside it.
            //.
            //. Example:
            //.   let y: Option<i32> = None;
            //.   Meaning: “y does not currently have a number”
            //.
            //. This is the Rust equivalent of null, BUT with a big safety difference:
            //. Rust makes you *handle* the None case before you can use the value.
            //. You cannot accidentally treat None like it's a real value.
    //. }

    //. The option enum is automaticlly introduced into every programs scope (You dont have to define it)
    //.^ Example: Working with Optional

    {
        //. Here we have two values, x is an integer and y is an optional integer (Can be null or it can have a value)
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        //. Lets see what happens if we try to add these values together

        let sum = x + y; //. We cannot add them as they are diffrent types

        //. Thus we need to first extract the value (in this case 5) out of Some so we have y = 5 instead of Some(5)
    
        //. For this we use .unwrap OR .unwrap_or()
        //. Lets first look at .unwrap_or()

        //. unwrap_or checks to see if the object it is being applied on has a value if it does .unwrap_or will send that value back. If it does not unwrap_or will send back the default value you called it with

        //. Example
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y.unwrap_or(0); //. If y has a value we send back that value, if y is null then we send back 0. So if y has a value y.unwrap_or(0) --> Gives us that value. If y does not have a value (is null) y.unwrap_or(0) --> gives us 0
    }

    //^ Using the match expression with enums
    //^ match allows you to compare a value against a set of patterns
    //^ ALWAYS USE MATCH IF WORKING WITH ENUMS

    //^ Example 1: Using enums with match

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => return 1,
            Coin::Nickel => return 5,
            Coin::Dime => return 10,
            Coin::Quarter => return 25,
        }
    }

    //. The paterns above can calso bind to values
    //^ Example 2
    {
        #[derive(Debug)] //. This makes it easier to print the UsState enum. (It allows us to be able to use {:?} when printing)
        enum UsState {
            Alabama,
            Alaska,
            Arizona,
            Arkansas,
            California,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState), //. Every Quarter now also holds the state it was minted in
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => return 1,
                Coin::Nickel => return 5,
                Coin::Dime => return 10,
                Coin::Quarter(state) => {
                    println!("The quarter is from {:?}", state);
                    return 25;
                },
            }
        }

        //. The above code uses Patern Matching to give state a value
        //. Here is a simple explanation:
        //. We defined it as Quarter(UsState) so now Quarter(state) is basicly saying take the value of UsState and put it in the state variable

        //^ if we called the above example like this value_in_cents(Coin::Quarter(UsState::Alaska)) 
        //^ The output would be: The quarter is from Alaska

        //^ The purpose of {:?} 
        //. It just allows us an easy way to apply debug formatting, (it will print what the object looks like)
        //. It helps us to print out more complex objects
        //. For example

        let v = vec![1, 2, 3];
        println!("{:?}", v); //. Will print [1, 2, 3]

        
    }

    //^ Now lets combine the match expression with our optional enum
    //. To do that we will create a function that takes in an optional integer and adds 1 to it
    {
        fn main() {
            let five = Some(5);
            let six = plus_one(five);
            let none = plus_one(None);
        }
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => return None, //. If x is None we return None
                Some(i) => return Some(i + 1), 
            }
            //^ Some(i) => Some(i + 1) EXPLANATION
            //. Lets first look at Some(i) since we are in a match it is saying
            //. If x is of the pattern Some(value) like Some(5) then take the value and put it in i
            //. Then return i + 1. Since our return type is Option<i32> we cant just return i + 1 since that would return an i32 we need to wrap it in a Some() so that we still return an Option<i32>
        }
    }

    //^ We can Simplify the above code using the if let syntax

    //. Example of if let (NOT SIMPLIFYING THE ABOVE CODE)
    {
        let x = Some(5);

        if let Some(3) = x {
            println!("X is 3")
        }

        //^ Explanation of if let Some(3) = x 
        //. It is doing 2 seprate things

        //. First it checks if x matches the pattern Some(value)
        //. If not the if gets skipped 
        //. If yes then it checks if x is equal to Some(3)

        //. So it is basicly doing this
        //. if (x matches Some(value)) {
        //.        if x == Some(3) {
        //.            println!("X is 3")
        //.        }
        //.  }

        //. (x matches Some(value)) is just a way of saying check the value of x. If it is Some(<any value>) then it matches the pattern Some(value)
    }
}
