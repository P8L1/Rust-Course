#![allow(warnings)] //^ Rust wont show warnings

//^ Generics are part of a 3-part series in Rust. (The series goes: Generics, Traits, Lifetimes)
//^ Right now we will only be looking at Generics. Next week we will look at Traits, and the week after that we will cover Lifetimes.

//^ Please note that this is a step up from "normal" programming. For many high-level languages, the learning curve effectively ends around here.
//^ This work is harder than normal (since its very abstract) and it is a bit unique to Rust. So blame yourself if you are struggling, and do not be afraid to ask for help. 

//^ Before we get into the ins and outs of Generics/Traits/Lifetimes you first need to understand DRY
//^ DRY is what girls are when they think of you. (Just a joke)
//^ In all seriousness DRY stands for Do not Repeat Yourself
//^ DRY is a fundamental principle of programming that we have touched without you even knowing it exists before.

//. Lets take functions as an example. Lets say you had to print out an entire vector 1000 times in your program
//. You could either A => Write the same for loop 1000 times
//.                              B => Write the for loop once in a function and call that function 1000 times with the vector you want to print out
//. Option A violates DRY since you are repeating the same code.

//. Generics/Traits/Lifetimes all help you to follow the DRY principles and they help you to write code that does not constantly repeat itself.

//^ To learn Generics lets first start off by reducing code duplication by extracting logic out into functions

mod example_one {
    fn main() {
        //. Here we have a vector called number list that contains a list of numbers
        let number_list = vec![34, 50, 25, 100, 65];

        //. Then we have some code below to find the largest number in the vector
        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is: {}", largest);

        //. This code works but what if we want to find the largest number in a different vector.
        //. We could just duplicate the code above like this: 

        let number_list_2 = vec![10006, 50, 25, 1000, 65];
        let mut largest = number_list_2[0];
        for number in number_list_2 {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is: {}", largest);

        //. But now we are in a direct violation of DRY since we are repeating ourself
        //. So lets extract the logic to find the largest number out into a function. (See example two)
    }
    
}

mod example_two {
    //. As you can see there is a lot less duplication now
    //. But now we have a problem, what if we want to apply the same logic but over a slightly different set of arguments.
    //^ Example: What if we want to find the char with the largest ASCII value in a vector of chars. Its similar code, and duplicating our largest_number function to work with a vector of chars would violate DRY.
    //^ See example 3
    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let  largest = largest_number(number_list);
        println!("The largest number is: {}", largest);

        let number_list_2 = vec![10006, 50, 25, 1000, 65];
        let  largest = largest_number(number_list_2);
        println!("The largest number is: {}", largest);
    }

    fn largest_number(v: Vec<i32>) -> i32 {
        let mut largest = v[0];
        for number in v {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
    
}

mod example_three {

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let  largest = largest_number(number_list);
        println!("The largest number is: {}", largest);

        let number_list_2 = vec![10006, 50, 25, 1000, 65];
        let  largest = largest_number(number_list_2);
        println!("The largest number is: {}", largest);

        //^ So instead of passing in our number_list we want to pass in a char_list
        //. Like so
        let char_list = vec!['y', 'm', 'a', 'q'];
        //^ Lets first solve this problem with duplication (See largest_char)
        //^ Now we can take the largest char function and use it like this
        let largest_char = largest_char(char_list);
        println!("The largest char is: {}", largest_char);
        //^ Now this works but notice how the logic in the functions largest_char and largest_number are basically the exact same.
        //^ Thus we are violating the DRY rules
        //^ Lets use Generics and make the code DRY compliant (see example 4)
    }

    fn largest_number(v: Vec<i32>) -> i32 {
        let mut largest = v[0];
        for number in v {
            if number > largest {
                largest = number;
            }
        }
        largest
    }

    fn largest_char(v: Vec<char>) -> char { //. A vector of chars is passed in and we send back a char
        let mut largest = v[0];
        for character in v {
            if character > largest {
                largest = character;
            }
        }
        largest
    }
    
}

mod example_four {
    //^ To use generics you first need to specify that your function uses Generics
    //# This can be done by placing <T> next to the function name. If your function takes more than one generic type as input you place <T, G> (You increase the letters the more generic types you use). You dont have to use the letter T or G specifically. Here T means Type and it is best practice for functions only taking one generic type as input to use T inside of the <>. If the function takes two generic types then you can choose any other letter to use along with T..
    //# So basically if the function only takes one generic type then you use T for anything more than that you can use whatever you want
    
    //^ Examples:

    //^ Example: Normal function
    fn normal(s: String) {
        println!("The string is {}", s)
    }

    //^ Example: One Generic
    fn one_generic<T>  (s: String) { //. One generic type is always T
        println!("The string is {}", s)
    }

    //^ Example Two generics
    fn two_generics<T, G> (s: String) { //. For more than one generic type you use T as the first value then you choose the letter for the rest
        println!("The string is {}", s)
    }
    //. Note: For more than two generics type you can just keep on adding letters in the <>

    //^ Now lets jump to the largest_number function (Ignore main, no changes were made in it)

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let  largest = largest_number(number_list);
        println!("The largest number is: {}", largest);

        let number_list_2 = vec![10006, 50, 25, 1000, 65];
        let  largest = largest_number(number_list_2);
        println!("The largest number is: {}", largest);

        //^ So instead of passing in our number_list we want to pass in a char_list
        //. Like so
        let char_list = vec!['y', 'm', 'a', 'q'];

        let largest_char = largest_char(char_list);
        println!("The largest char is: {}", largest_char);

    }

    //^ Changes made
    //. 1. Added T to tell rust we will be using one generic type (One variable can be either a  Vec<i32> or Vec<char>)
    //. 2. Removed Vec<i32> and changed it to T. So we now receive input that is of type T
    //. 3. Removed -> i32 since we are not returning an integer we are returning whatever the Generic type (T) is
    fn largest_number<T>(v: Vec<T>) -> T {
        let mut largest = v[0];
        for number in v {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
    //. Now we get the error that we cant use the operator > on our type T. This makes sense since T can be anything. For example a struct. If T is a struct then it wont be possible for rust to apply > 
    //. Thats why we need to limit T to certain types see example_five
}

mod example_five {
    //^ So we want to basically tell Rust limit T to be any type that can be compared. In order to do this we need to use Traits
    //^ We will be learning about traits next week, so lets just fix it with traits now, and next week we will go deeper into it
    //^ Jump to the get_largest function first

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let  largest = get_largest(number_list);
        println!("The largest number is: {}", largest);

        let number_list_2 = vec![10006, 50, 25, 1000, 65];
        let  largest = get_largest(number_list_2);
        println!("The largest number is: {}", largest);

        //^ So instead of passing in our number_list we want to pass in a char_list
        //. Like so
        let char_list = vec!['y', 'm', 'a', 'q'];

        let largest_char = get_largest(char_list);
        println!("The largest char is: {}", largest_char);

    }

    //^ I fixed it with traits by replacing <T> with <T: PartialOrd + Copy>
    //. This is basically telling rust that our type T needs to be a type that can be Ordered and Copied (Copied means you can use let x = y without moving the value think of integers and chars)
    //. Don't worry to much about this since we will learn about it in depth later
    //^ I also reamed the function to get_largest
    //^ Lets look at how we can use this function in main
    fn get_largest<T: PartialOrd + Copy>(v: Vec<T>) -> T { 
        let mut largest = v[0];
        for number in v {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
}


//^ Now lets look at using Generics with structs

//. Here we have a struct called point with two fields, x and y. Both x and y are 32  bit signed integers
//. Then in main we create a new Point
mod generics_with_structs_example {
    struct Point {
        x : i32,
        y : i32,
    }

    fn main() {
        let p1 = Point { x: 5, y: 10};
        //. But what if we create a new point with floating point numbers instead of integers
        let p1 = Point { x: 5.00, y: 10.0};
        //. This code obviously throws an error.
        //. Generics can help us here as well, refer to generics_with_structs_example_2
    }
}

mod generics_with_structs_example_2 {
    //. Added Generics to the struct
    struct Point<T> {
        x : T,
        y : T,
    }
    fn main() {
        //. Our code now works perfectly
        let p1 = Point { x: 5, y: 10};
        let p1 = Point { x: 5.00, y: 10.0};
        //. But now I have a problem I want to make a point where x is a i32 and y is a f64
        let p1 = Point { x: 5, y: 10.0};
        //. This currently throws an error but we can fix it with generics as well, refer to generics_with_structs_example_3
    }
}

mod generics_with_structs_example_3 {
    //. Added another generic type G 
    //. So now x is of the generic type T
    //. And y is of the generic type G, thus both x and y can be different types or the same type
    struct Point<T, G> {
        x : T,
        y : G,
    }
        fn main() {
        let p1 = Point { x: 5, y: 10};
        let p1 = Point { x: 5.00, y: 10.0};
        let p1 = Point { x: 5, y: 10.0};
    } //^ We could also use Generics inside of enums, check generics_with_enums_example_1
}

mod generics_with_enums_example_1 {
    //. The two most popular enums we have been using in the past weeks actually use generics
    //. The option enum
    enum Option<T> {
        Some(T),
        None,
    }
    //. The result enum
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    //. Lets see how we can define functions on structs or enums with generics, refer to generics_with_structs_functions
}

mod generics_with_structs_functions {

    struct Point<T> {
        x: T,
        y: T,
    }
    //. Please note that one struct can have more than 1 implementation block
    //. Here our point struct takes a generic type T and both x and y are of the type T
    //. Now lets define a method on our point struct
    //. We want our implementation block to use generics so we will write impl<T>
    impl<T>  Point<T> {
        //. Lets declare a method called x
        //. X takes a reference to self which is the point instance we are operating on and then  returns a reference to the x field
        fn x(&self) -> &T {
            &self.x
        }
    }
    //. Note the Generics you specify in your impl block don't have to be the same as the Generics specified when creating the struct
    //. This works the same as lines 302 to 308
    impl<U>  Point<U> { //. So basically the type parameter could be anything
        fn x_v2(&self) -> &U { //. Renamed from x to x_v2 sp we don't get a double declaration error
            &self.x
        }
    }
    //. To make this even clearer lets define another implementation block on point with a concrete type parameter
    impl Point<f64> { //. So here we are defining an implementation block for Point and it is only for points that have a f64 type parameter 
        fn y(&self) -> f64 { //. So here we define a method that returns the y value of the point called on
            self.y
        }
    }
    //. point_object.x(); is available to all Point instances but point_object.y(); is only available for points where both x and y are of type f64
    //. Lets demonstrate the concept defined in the previous line
    fn main() {
        //. Lets create a point called p1 that has both x and y as integers
        let p1 = Point{
            x: 5,
            y: 10
        };
        //. Now lets create a point called p2 that has both x and y as floating point numbers
        let p2 = Point{ 
            x: 5.5,
            y: 90.9
        };

        //^ TO DEMONSTRATE
        //. point_object.x(); is available to all Point instances but point_object.y(); is only available for points where both x and y are of type f64
        p1.x(); //. Works since x is available for all Point instances
        p2.x(); //. Works since x is available for all Point instances

        p1.y(); //. Does not work since p1.x and p1.y are not of the type f64
        p2.y(); //. Works since both p2.x and p2.y are of type f64

        //. Now lets go to a more complex example, refer to more_complex_example
    }
}

mod more_complex_example {

    //. Here we have our point type again except now we have two generics, T and U
    struct Point<T, U> {
        x: T,   //. So x is going to be one type (T)
        y: U,   //. And y is going to be another type (U), Note they can still be of the same type, then both T and U will be that type. So both T and U can still be i32's, they dont need to be different types
    }

    //. Then we define an implementation block with some generics (T and U), and we will be implementing for our point struct
    impl<T, U> Point<T, U> {
        //. Now we define a method called mixup with its own Generics V and W, both V and W are scoped to the mixup function. 
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> { //. Mixup takes self as the first argument and the second argument is called other and it is of type Point. This Argument will be using the V and W generics defined next to the mixup function name. Why did I use V and W instead of T and U? We want the other instance of Point to be able to have parameters that are of different type than the Point object the method is called on . Our return type is also a Point and there we mix up the Generics, So first we have T which comes from the Point object we are calling the method on and then W which comes from the point that is passed into the function
            //. Refer to main to see this method in action
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        //. First we declare an instance of the Point struct called p1, that has x as a i32 and y as a f64
        let p1 = Point {
            x: 5,
            y: 10.4,
        };
        //. Then we declare another Point called p2, its x value is a string slice (&str) and the y value is going to be a char
        let p2 = Point {
            x: "Hello",
            y: 'c',
        };
        //. Finally we declare a Point called p3 and its value is going to be set to the output of the mixup method applied on p1
        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y); //. Will Print: p3.x = 5, p3.y = c
    }

    //^ The last thing I want to talk about is performance, Generics are great because they allow us to reduce duplication. Refer to generics_performance
}

mod generics_performance {

    //. I redefined the option enum for clarity
    enum Option<T> {
        Some(T),
        None,
    }

    fn main() {
        //. In this case we have two instances of the Option enum, one is a float and the other an integer. 
        //. Instead of defining two version of the Option enum (one for i32 and one for f64), we can use a generic for the some variant.
        //. Luckily for us the use of a Generic DOES NOT IMPACT PERFORMANCE. Thats because at compile time Rust will actually turn the option enum into two option enums one for i32 and one for f64. (So Rust will Generate all the necessary versions of the thing that uses Generics at compile time)
        let integer = Option::Some(5);
        let float = Option::Some(5.0);
        //. So at compile time it will look something like this, refer to at_compile_time
    }
}

mod at_compile_time {
    //. This is what rust will do to mod generics_performance when you compile
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32 ::Some(5);
        let float = Option_f64 ::Some(5.0);
    }
}

fn main() {
    
}
