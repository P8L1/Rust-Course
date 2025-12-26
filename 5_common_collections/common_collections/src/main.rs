#![allow(warnings)] //. Stops the compiler from generating warnings

//^ Collections allow you to store multiple values in one variable. But unlike arrays and tuples collections are stored on the Heap and not the stack. Meaning the size of the collection can grow or shrink as needed. 

//^ We will be talking about
//^ 1. Vectors
//^ 2. Strings
//^ 3. HashMaps

//^ Lets first do a super low level lesson

//~ What is binary
//. Binary is a number system that uses only two digits: 0 and 1. Computers love it because 0/1 maps nicely to "off/on" in electronics.

//~ What is a bit
//. A bit is one binary digit - either 0 or 1
//. So a bit can represent 2 possibilities

//~ What is a byte
//. A byte is a group of 8 bits, a byte can thus represent 2^8 diffrent patterns (From 00000000 to 11111111)

//^ Rust integer types: i8, i32, i64

//. In Rust, the i means signed integer ("can be negative or positive").
//. The number (8, 32, 64) is the bit-width: how many bits are used to store the value.

//~ i8

//. 8 bits total = 1 byte
//. Range: -128 to 127
//. Small, efficient, often used for byte-like numeric data.

//~ i32

//. 32 bits = 4 bytes
//. Range: -2,147,483,648 to 2,147,483,647
//. This is the default integer type in Rust when the compiler can infer it (e.g., let x = 5; often becomes i32).

//~ i64

//. 64 bits = 8 bytes
//. Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

//~ Mental model
//. More bits => more possible patterns => bigger range

//~ Examples
//# let a: i8 = 120;        // ok
//# let b: i32 = 2_000_000; // ok
//# let c: i64 = 9_000_000_000; // ok
//# let bad: i8 = 200;   //  too large for i8

fn main() {

    //^ ==================================================== Vectors ====================================================

    let a = [1, 2, 3]; //. Here I created an array of signed 32 bit integers, Lets do the same thing but for vectors
    let mut v: Vec<i32> = Vec::new(); //. Here we are calling the new function on vector, it returns an empty vector
    //. Adding items to a vector
    v.push(1); //. Appends 1 to the vector
    v.push(2); //. Appends 2 to the vector
    v.push(3); //. Appends 3 to the vector

    //. Initialising a vector with values

    let v2 = vec![1, 2, 3]; //. This creates a vector with the values 1 2 and 3 inside
    //. Note how I did not define the type explicitly. Rust was able to infer the type since I defined it with values
    //. Note that vectors just like any other type stored on the heap will be dropped when they go out of scope.
    {
        let v3 = vec![1, 2, 3];
    } //. v3's scope ends here and it is dropped

    //. Acessing elements in a vector

    //. There are two ways to acess a specific element in a vector

    //. The first way is to directly refrence an index in the vector

    let v4 = vec![1, 2, 3, 4, 5];
    let third_element_of_v4 = &v4[2];
    println!("The third element is {}", third_element_of_v4); //. Will print 3

    //. The problem with the above approuch is that we can specify an invalid index for example instead for 2 lets put 20
    let error_code = &v4[20]; //. the vector does not have a 20th element so thus this code will throw a runtime error
    //. If we used an array and tried to acess an index out of bounds we would get a compile time error
    //. Lets say you dont want your program to crash if an invalid index is used

    //. In that case rust provides a safer way to acess elements in a vector using the get method

    let x = v4.get(2);

    match x {
        Some(v4_3rd_element) => print!("The third element in the vector v4 is {}", v4_3rd_element),
        None => println!("There is no third value"),
    }

    //. When working with vectors in the future we will expect of you to ALWAYS use the .get method to retrieve a certin element in a vector.
    //. This is done to ensure that your code handles runtime errors gracefully

    //. Now when we acess a certin element in a vector we automaticly get a refrence to that element.
    //. Borrow checking still applies (Cant have mutable and immutable refrence at the same time)

    let mut test_vector2 = vec![1, 2, 3];
    let third_element = &test_vector2[2];
    test_vector2.push(3);  //. If we have an immutable refrence to something (In this case test_vector2) we expect the underlying value to not change. But test_vector2.push() creates a mutable refrence allowing the underlying value to change
    println!("The refrence third element goes out of scope here, its value is {}", third_element);

    //^ Itaraating over vectors
    //. The standard way of iterating over a vector is via a for each loop. For each loops are iteratave loops (Itarate for loops are the best types of loops as there are no indexing errors)

    let vector1 = vec![1, 2, 3, 4];

    for x in &vector1 { //. You can replace x with any variable name. This goes though the vector assigns a element to x then executes the for loop 
        println!("{}", x);
    }

    //. So the above code will print
    //. 1
    //. 2
    //. 3
    //. 4
    
    //^ Using vectors with enums 
    //. Vectors can only store one type of data, but lets say we want to store diffrent types of data

    //. Example: We want a vector to represent a row of cells in a spreadsheet, each cell could store either a integer, floating point number, or a string
    //. In order to represent this in a vector we could create an enum that represents a cell

    enum SpreadsheetCell {
        //. Now we create variants of SpreadsheetCell for each type of data
        //. Now remember that all of these variants are under the same type which in this case is SpreadsheetCell
        Int(i32),
        Float(f64),
        Text(String),
    }

    //. Now we can create a vector of the diffrent types 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hi")),
        SpreadsheetCell::Float(10.12),
    ];

    //. Now the only catch is when you refrence a specific element inside of the vector you have to use a match expression to figure out which variant of the enum it is
    //. Example - if its an int print it, if its anything else print Not a integer

    match row.get(1).expect("shit") {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer"),
    }


    //^ ==================================================== Strings ====================================================

    //. Strings in Rust are pretty dam complicated.

    //~ Forword: What is ASCII and what is UTF 8

    //. In 1963, a little teleprinter named Tess worked the night shift at a busy communications office. Every message she sent was just electrical clicks, so her boss handed her a brand-new “alphabet” called ASCII—the American Standard Code for Information Interchange. It was published as a standard in 1963, and it gave Tess a simple rulebook: each letter, number, and symbol gets a 7-bit number—which means 128 total characters (A–Z, a–z, 0–9, punctuation, plus control codes like newline). 
    //. Years passed. Tess’s great-grandkid, a modern computer named Nova, got messages from everywhere: “你好”, “مرحبا”, “🙂”, and thousands more symbols that ASCII simply didn’t have room for. Nova needed a bigger universal alphabet (Unicode), but still wanted to keep old ASCII messages readable.
    //. So engineers created UTF-8: a clever way to store Unicode characters using bytes, where common characters stay small and rare ones can take more space. UTF-8 was devised in September 1992 (Ken Thompson, guided by Rob Pike’s design criteria), and it was later standardized in IETF documents (one widely used modern spec is RFC 3629, published in 2003). UTF-8 is variable-length (1 to 4 bytes), and its best magic trick is this: the first 128 characters are exactly the same as ASCII, so old English-only text still works perfectly.

    //~ TL DR

    //. ASCII => Each char is represented by 1 byte
    //. UTF-8 => A char can be represented by 1 byte, 2 bytes, 3 bytes or 4 bytes
    //. So in UTF-8 each char can be a diffrent size in terms of bytes
    //. UTF-8 is the most popular encoding of unicode, so thats why in rust we use UTF-8 as well

    //~ What are string actually

    //. In rust Strings are simpily a collection of UTF-8 encoded bytes

    //. Examples of creating strings in Rust

    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    //. Strings are UTF-8 encoded so we can write them in english as well as in many other langauges 

    let hello = String::from("Hello");
    let hello = String::from("你好");
    let hello = String::from("Salve");
    let hello = String::from("مرحبا");
    let hello = String::from("Γειά σου");

    //. Appending to a string in Rust
    //. Just like a vector, a string can grow or shrink in size
    let mut s = String::from("foo");
    s.push_str("bar"); //. We use the push_str() method to push bar to the string. push_str takes in a string slice because we dont actually want to take ownership of the string that was passed in
    //. We can also append strings using the push method
    s.push('!'); //. The push method can only take a character NOT A STRING to append it to the underlying string
    //. The value of s is now foobar!

    //. We can also append strings using the plus operator

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //. Moves ownership of s1 to s3 then takes all the chars from s2 and appends them at the end

    //. We can achieve the same thing using the format macro without taking ownership
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2); //. No ownership changes occured

    //. Indexing into a string
    //. Lets say we want the first char of a string

    //. In higher level programming langauges you can just do this 
    let hello = String::from("Hello");
    let first_char = hello[0];

    //. In Rust, this gives an error? Why is this?
    //. A string is a collection of bytes.
    //. Whats the lenght of our hello string, you might say 5 since there are 5 chars. That would be correct since every char in "Hello" is one byte
    //. But lets change hello to Γειά σου which is hello in greek
    let hello = String::from("Γειά σου"); 
    let first_char = hello[0];
    //. Now whats the lenght of this string "Γειά σου", you might say 8 since you see 8 chars, but in reality it is 15  since every Greek char is stored as 2 UTF-8 bytes (so 7 times 2 = 14 + space (one byte) = 15)
    //. Since the first char is two bytes hello[0] would only give us half of the character

    //. To better understand this lets look at this example

    let namasty_hindi = String::from("नमस्ते");

    //. Three relavant ways to represent this word in Unicode

    //. 1.  Bytes: 
    //. [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] 
    //. 2. Scalar values: (Building blocks in Unicode, they can represent a full char or parts of a char. This is what the rust char type refers to)
    //. ['न', 'म', 'स', '्', 'त', 'े'] (This is a vector of chars)
    //. 3. Grapheme clusters: (What you and I would consider a char)
    //. ["न", "म", "स्ते"]

    //~ The problem with indexing a string is that rust does not know what we want to recieve, bytes, Scalar values or Grapheme clusters. So thus we need to use more specific methods
    
    //. Example of itarating over a string using each method

    //. For bytes use:
    for byte in namasty_hindi.bytes() {
        println!("{}", byte);
    }

    //. For Scalar values use:
    for character in namasty_hindi.chars() {
        println!("{}", character);
    }

    //. For Grapheme clusters things are a lot more complex. In order to keep the Rust langauge and Standard Library lean the ability to fetch a strings Graphime clusters in not included by default
    //. To iterate over graphine clusters you need to:
    //. 1. Add unicode-segmentation to cargo.toml (as a dependency. Note I did so for this project)
    //. 2. Bring it into scope
    use unicode_segmentation::UnicodeSegmentation;
    //. 3. Use this code

    for grapheme in namasty_hindi.graphemes(true) {
        println!("{}", grapheme);
    }

    //. Example to get the value of a specific index of a String
    let index = 1;

    //. For bytes use:
        let b: u8 = *namasty_hindi
            .as_bytes()
            .get(index)
            .expect("byte_index out of range");

    //. For Scalar values use:
        let ch: char = namasty_hindi
            .chars()
            .nth(index)
            .expect("scalar_index out of range");

    //. For Grapheme clusters use:
        let g: &str = namasty_hindi
            .graphemes(true)
            .nth(index)
            .expect("grapheme_index out of range");

     //^ ==================================================== Hashmaps ====================================================

    //^ What is a HashMap

    //~ A HashMap is a datastructure that is used to store key value pairs in Rust. (The keys and values can be of any type)
    //~ It uses a Hashing function to determine how to store those keys and values in memory

    //. What a HashMap looks like visually (Example)
    //     SIMPLE_JWT_HashMap = {
    //          "ALGORITHM": "RS256", //. The key is ALGORITHM and the value is "RS256" 
    //          "SIGNING_KEY": JWT_PRIVATE_KEY, //. The key is SIGNING_KEY and the value is a variable called JWT_PRIVATE_KEY
    //          "VERIFYING_KEY": JWT_PUBLIC_KEY, //. Same logic as above
    //          "ACCESS_TOKEN_LIFETIME": 3600,
    //          "REFRESH_TOKEN_LIFETIME": 360000,
    //          "ROTATE_REFRESH_TOKENS": True,
    //          "BLACKLIST_AFTER_ROTATION": True,
    //          "AUTH_HEADER_TYPES": "Bearer",
    //      }

    //^ Creating a new HashMap in Rust 

    //^ Example for creating a game related HashMap

    //. First we need to bring the HashMap type into scope from the standard library
    use std::collections::HashMap;

    //. Next let's define some keys
    let blue_team = String::from("Blue Team");
    let yellow_team = String::from("Yellow Team");

    //. Now lets create our HashMap
    let mut scores = HashMap::new(); //. Our HashMap is named scores and just like with vectors and strings we can use the new function

    //. Now lets populate our HashMap
    scores.insert(blue_team, 10); //. Note: This takes ownership of blue_team
    scores.insert(yellow_team, 50); //. Note: This takes ownership of yellow_team

    //. We could pass in a refrence to our strings but that would require the use of lifetimes which we will talk about later

    //^  Getting individual values out of a HashMap by using the get method
    //. The syntax for this is
    //. <HashMap_name>.get(<Key_Name>);
    
    //^ Example
    let team_name = String::from("Blue Team");
    let blue_team_score = scores.get(&blue_team);

    //. The get method takes in a refrence to a string (The name of the key) then it returns an optional value
    //. The get method rteturns an optional value since it cant gaurentee that a value will be returned

    //. We can also Iterate over all the elements in our hashmap

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    //^ Updating values in a HashMap

    //^ Example with teams

    let mut scores = HashMap::new();

    let blue_team = String::from("Blue Team");
    let yellow_team = String::from("Yellow Team");

    scores.insert(blue_team, 10);
    scores.insert(blue_team, 50);  //. This will ovveride the blue key with the value 50

    let blue_team = String::from("Blue Team");
    let yellow_team = String::from("Yellow Team");

    //. Hoewever if we dont want to override existing values we can use this syntax 
    scores.entry(yellow_team).or_insert(30);
    scores.entry(yellow_team).or_insert(60);
    //. First we call scores.entry() -> Gives us the value for a given key (enum). Then we can call methods on that enum.
    //. In this example we call the .or_insert method
    //. What .or_insert does:
    //. If there is no entry for the yellow_team key then insert a new entry for yellow with the value of 30
    //. But on the second line (The one where we attempt to insert 60) yellow does exist  so we dont do anything
    
}
