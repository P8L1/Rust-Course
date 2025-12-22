#![allow(warnings)] //. Stops the compiler from generating warnings

//^ Structs basics

//^ Declaring a struct

struct User { //. Creates a struct with the name User
    username: String,   //. Datafields are seperated by commas
    email: String,      //. The values username, email, sign_in_count and active can be refred to as fields of User or as attributes of User
    sign_in_count: u64,
    active: bool,
}

//^ Like a tuple our struct allows us to group related data. But we can name the type (In this case User) and we can also name the data in our struct

//@ The data (also called data fields) in the above struct is
//~ username, email, sign_in_count, active

//^ Unlike tuples structs allow us to refrence our data by name instead of index location

fn main() {




    //. The variable user1 is of the TYPE user. user1 is also an "instance" of the User struct, in short user1 is an instance of User
    let mut user1 = User {
        email: String::from("test@test.com"), //. The order we define attributes is not the same as the order they were defined when we first created the struct. But still we need to give a value to every attribute we defined when we created the struct.
        username:  String::from("test"), //. Note values are still comma seperated
        active: true,
        sign_in_count: 1,
    }; //. We still end with a ;



    //^ Lets create a variable called name that has the value of user1's name field 
    //. The syntax for retrieving a specific value from a struct is <variable name>.<attribute name>
    //. So thus the code looks like this
    let name = user1.username; //. user1 is the variable's name, and username is the attribute/field name



    //^ Lets change the value of our user1's email attribute
    user1.email = String::from("hi@test.com"); //. We still use the <variable name>.<attribute name> syntax
    //. Note for the line above to work we need to make the entire struct variable (User1) mutable. You cant just make the email field mutable



    //^ We can also use functions to create new instances of User.
    //. Example. Remember the goal is to make a function we just call with an email and a username then it sends back a User instance we can use in the future
    {
        struct User { 
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        fn main() {
            let user = String::from("Hi");
            let email_adress = String::from("Hi@test.com");
            let user2 = build_user(
                email_adress,
                user
            );
        }
        fn build_user(email: String, username: String) -> User { //. The value we return is of the type User
            
            //~ Option 1, bad code
            //. We could write something like this:
            // return User {
            //          email: email, //. We take the email value that the function was called with and use it to give a value to the instances email attribute. (Take the value of the variable email which the function was called with, and use it to give the email field a value)
            //          username: username,
            //          active: true,
            //          sign_in_count: 1,
            //        }
            //. But this code can be cleaner.

            //~ Option 2, cleaner code 
            return User {
                email,
                username,
                sign_in_count: 1,
                active: true,
            };
            //. The above code is basicly saying: Take the email variable passed into this function and use it to give the email attribute of User a value (same for user)
            //. Then we just manually assign values to active and sign_in_count
            //. This syntax where we just use the variable passed into the function as both a name for the attribute we are defining and a value for the attribute we are defining is called the Field Innit Shorthand Syntax
        }
    }



    //^ We can also create new instances of a struct using existing instances
    //^ Example
    {
        struct User { 
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        fn main() {
            //^ Lets create our first user
            let userA = User {
                username: String::from("userA"),
                email: String::from("userA@test.com"),
                sign_in_count: 1,
                active: false,
            };

            //^ Now lets use this first user to create a new user. I want the new user to have the same values for sign_in_count and active as userA 
            let userB = User {
                username: String::from("userA"),
                email: String::from("userA@test.com"),
                ..userA //^ This tells rust: "For the remaining values just use whatever we have for userA"
            };
            //. The above syntax (The ..userA) is called the struct update syntax
        }
        
    }



    //^ We can also make structs without named fields and these are called tuple structs
    //^ Example
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    //. Tuple Structs are useful when you want your entire tuple to have a name and be of a diffrent type than other tuples. Color and Point both have the same fields (Both have 3 32 bit integers). But since they are Structs (they are their own types) if a function expects an input that is of type Color you cant accidently send it a value of type point. (With regular tuples this can happen)


    //^ Implimenting your own methods on a struct
    //. Introduction: You have certinly used the .to_lowercase() or the .to_string() methods before. (Yes they are called methods, because they are applied on an object.)
    //. The object a method is applied on is whatever is before the . so for example "hi".to_string() the method is to_string() and the method is being applied on the object "hi". "hi" is an instance of rusts built in String Struct!

    //^ Lets impliment our own methods.

    //. Lets define a struct
    struct Rectangle {
        height: u32,
        width: u32,
        lenght: u32,
    }

    //. Lets define an instance of the struct
    let fish_tank = Rectangle {
        height: 20,
        width: 30,
        lenght: 40,
    };

    //. Now to define methods we start an impliments block for the struct we want to define methods for. In this case I want to define an area method for the Rectangle struct
    impl Rectangle { //. Use the impl keyword followed by the struct name we want to impliment for

        //. Note:

        //. More than 1 method can be defined in one impl block
        //. All methods are functions but not all functions are methods. You know something is a method if it takes the self keyword
        //. self just means your going to be sending the method an instance of the struct the method is implimented for.
        //. So in this case self means that we will be calling the method with an instance of Rectangle

        //. Lets impliment 2 methods for Rectangle, volume and area

        fn volume(&self) -> u32 { //. We take self as input (A refrence to self) and we send back a u32
            return self.height * self.lenght * self.width;
        }

        fn area(&self) -> u32 {
            return 4 * (self.height * self.width) + 2 * (self.lenght * self.width);
        }

        //. Lets also impliment another method called bigger than. It gets applied on one rectangle and it reecieves another as input. Then returns a bool.

        fn bigger_than(&self, rectangle_to_compare_to: &Rectangle) -> bool {
            let is_bigger_than = self.area() > rectangle_to_compare_to.area();
            return is_bigger_than;
        }
    }

    //^ Calling the methods IMPORTANT

    let volume_of_fish_tank = fish_tank.volume();
    println!("The volume of the fish tank is {}", volume_of_fish_tank);

    let area_of_fish_tank = fish_tank.area();
    println!("The area of the fish tank is {}", area_of_fish_tank); //. Run the program and check if it works!

    //. Rust automaticly puts the object the method is applied on in self


    //. Lets use the bigger than method

    let rect1 = Rectangle {
        height: 20,
        width: 30,
        lenght: 40,
    };

    let rect2 = Rectangle {
        height: 30,
        width: 30,
        lenght: 40,
    };
    
    let rect1_bigger_than_rect2 = rect1.bigger_than(&rect2); //. The bigger than function gets self from the object it is applied on (in this case rect1) then we need to send it rect2
    //. The value of rect1_bigger_than_rect2 is false

    //^ Associated functions
    //. Associated functions dont recieve the self parameter

    struct Rectangle_2D {
        lenght: u32,
        width: u32,
    }

    //. Lets create a Associated function that recieves the parameter size and sends back an instance of Rectangle_2D
    //. Associated functions are also defined in the impl block, they just dont recieve the self parameter

    impl Rectangle_2D {
        fn build(size: u32) -> Rectangle_2D {
            return Rectangle_2D {
                lenght: size,
                width: size,
            };
        }
    }

    //^ Calling Associated functions

    let new_rectangle = Rectangle_2D::build(20); 

    //. New rectangle is now an instance of Rectangle_2D and it has a lenght of 20 and a width of 20
}




