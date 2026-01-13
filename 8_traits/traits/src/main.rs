#![allow(warnings)]
//^ Firstly I would like to congratulate you on completing 20% of the language
//^ Traits in Rust are used to define shared behavior, Refer to program_one

mod program_one {
    //^ Here we have a program which aggregates different types of data's text content
    //. In this case a NewsArticle and a Tweet
    //. Now what we want is the ability to summarize a NewsArticle and also the ability to summarize a Tweet, so we can post it in our Text arrogation feed. In this case we can use a trait a define the shared behavior between a tweet and a news article
    //. To get more specific by shared behavior I mean methods, traits allow us to define a set of methods that are shared across different types

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    //^ Lets define a summary Trait
    //. We use pub since we want the Trait to be public
    pub trait Summary {
        //. Now inside the Summary trait we can define the shared methods
        //. Lets define a new method called summarize
        fn summarize(&self) -> String;
        //. Notice here we only specified the method signature, we dont actually have a method body
        //. Thats because we dont want to dictate an implementation, we just want to say for every type that has this trait  they should have this summarize method and it will return a string
        //. Now lets implement the summary trait for our NewArticle type, refer to mod implementing_summary
    }
    fn main() {}
}

mod implementing_summary {

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    //. So we will start by creating a new implementation block
    //. The syntax for implementing a trait for a type is
    //. impl <trait_name> for <type_to_implement_trait_for> {}
    impl Summary for NewsArticle {
        //. Now we will specify our implementation of the summarize method
        //. So we are saying:
        //. 1. Implement the summary trait for our NewsArticle type
        //. 2. For our summarize method return a string that contains the headline of the NewsArticle and the author of the NewsArticle
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    //. Now lets define the summary trait for our Tweet type

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }
    fn main() {
        //. Now in main we create a Tweet and an article with some random data

        let tweet = Tweet {
            username: String::from("@johndoe"),
            content: String::from("Hello World!"),
            reply: false,
            retweet: false,
        };

        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };

        //. Now we can call summarize on both the tweet and the article

        println!("Tweet summary: {}", tweet.summarize()); //. Will print: @johndoe: Hello World!
        println!("Article summary: {}", article.summarize()); //. Will print: The Sky is Falling!, by John Doe
    }

    //^ Now lets talk about default implementations, refer to default_implementations
}

mod default_implementations {

    //. Lets say that instead of just defining the summarize method signature and expecting every type that implements this trait to specify the body of summarise we want a default implementation
    //. To do that we can simply add a method body like so
    pub trait Summary {
        //. Here we are saying that our Summary trait has a method called summarize and its default implementation is to return a read more string
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    //. Our NewsArticle Struct is overriding the implementation from line 108, and thus <news_paper_instance>.summarize() will still return format!("{}, by {}", self.headline, self.author)

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    //. To demonstrate overriding we will remove the contents of our implementation block for Tweet
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {} //. No methods defined

    fn main() {
        let tweet = Tweet {
            username: String::from("@johndoe"),
            content: String::from("Hello World!"),
            reply: false,
            retweet: false,
        };
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Tweet summary: {}", tweet.summarize()); //. Will now print: (Read more...)
        println!("Article summary: {}", article.summarize()); //. Will still print: The Sky is Falling!, by John Doe
    }

    //^ Default implementations can call other methods inside our trait definition refer to di_call_other_methods
}

mod di_call_other_methods {

    //. Instead of having one function summarize we will have two
    pub trait Summary {
        //. The first method is called summarize author and it has no default implementation
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    //. We need to specify summarize_author since it does not have a default implementation
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    //. We dont need to specify summarize since we have  a default implementation
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    fn main() {
        let tweet = Tweet {
            username: String::from("@johndoe"),
            content: String::from("Hello World!"),
            reply: false,
            retweet: false,
        };
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Tweet summary: {}", tweet.summarize()); //. Will now print: (Read more from @johndoe...)
        println!("Article summary: {}", article.summarize()); //. Will still print: The Sky is Falling!, by John Doe
    }
    //^ Now lets look at Trait Bounds AKA traits as parameters, refer to trait_bounds
}

mod trait_bounds {

    //. Here I have a new function called Notify, and it takes in one item. This item is a reference to something that implements Summary
    //. So basically were saying item could be anything/any type that implements summary
    //. Now lets jump to main
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize())
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    fn main() {
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Article summary: {}", article.summarize());

        //. Here I call notify with a reference to article
        notify(&article); //. Will make Rust print out: Breaking news! (Read more from John Doe...)
        //. jump to trait_bounds_proper when you understand the above code
    }
}

mod trait_bounds_proper {

    //. The previous impl syntax works for straight forward cases but it is actually a syntax trigger for something called a trait bound
    //. pub fn notify(item: &impl Summary) { //. The previous impl syntax, commented out to avoid errors
    //.     println!("Breaking news! {}", item.summarize())
    //.  }

    //. Here we have a Generic T that is followed by : Summary
    //. The : Summary tells rust that this generic is limited to something that implements the summary trait.
    //. Although this is a longer form of the function above using the impl syntax (line 263), it is Generally cleaner especially for more complex code.
    //. Thus I expect you to use it instead of the impl syntax from now on
    //^ Lets look at a slightly more complex example where trait bounds give us slightly more power, refer to complex_trait_bounds
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize())
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    fn main() {
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Article summary: {}", article.summarize());
        notify(&article);
    }
}


mod complex_trait_bounds {

    //^ Here is another example of why the impl syntax is bad

    //. Here we have a notify function that takes two params, item1 and item2 both of which need to be of any type that implements the Summary trait, but what if we want both item1 and item2 to have the same type
    pub fn notify(item1:  &impl Summary, item2: &impl Summary ) {
        // ....
    }

    //. The &impl syntax is of no help here since they can both be of any type (Including different types) as long as the type implements Summary
    //. Thus we need to use Proper trait bounds via Generics
    pub fn notify_v2<T: Summary>(item1: &T, item2: &T ) { //. Changed the name to notify_v2 to avoid double declaration errors
        // ....
    }

    //. notify_v2 says that both parameters need to be of type T which has to implement the Summary trait, but since both are type T both also need to be the same type

    //^ Note that we could also specify multiple traits, refer to multiple_traits_example
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    fn main() {
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Article summary: {}", article.summarize());
        notify(&article, &article);
    }
}

mod multiple_traits_example {

    //^ Multiple traits using the impl syntax
    //. Now item1 needs to be of a type that implements Summary and Display
    pub fn notify(item1:  &(impl Summary + Display), item2: &impl Summary ) {
        // ....
    }

    //^ Multiple traits using the trait bound syntax
    pub fn notify_v2<T: Summary + Display>(item1: &T, item2: &T ) { //. Changed the name to notify_v2 to avoid double declaration errors
        // ....
    }

    //^ One last thing to note: Specifying Multiple trait bounds could hinder readability, refer to hindered readability

    pub trait Display {}
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    impl Display for NewsArticle {}
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    fn main() {
        let article = NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is falling!"),
            content: String::from("The sky is not actually falling"),
        };
        println!("Article summary: {}", article.summarize());
        notify(&article, &article);
    }
}

mod hindered_readability {
    //^ Example of traits hindering readability

    use std::fmt::Display;
    use std::fmt::Debug;

    //. So here we have a function called some_function, it has a Generic type T that implements Display and clone AND it also has a generic type U that implements clone and debug.
    //. As you can see there is a lot of text between the function name and the function parameters.
    //. This can become problematic, to fix it we can use something called the where clause
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        6
    }

    //^ Example of the where clause
    fn some_function_fixed<T, U>(t: &T, u: &U) -> i32 
        where T:  Display + Clone,
                    U: Clone + Debug
    {
        6
    }

    //^ Now lets talk about return types implementing traits, refer to r_implementing_traits
}

mod r_implementing_traits {
    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    //^ The return type is impl Summary so we return any type that implements the summary trait
    fn returns_summarizable() -> impl Summary {
        NewsArticle {
            author: String::from("Probably ChatGPT"),
            content: String::from("As an AI Language model.."),
            headline: String::from("Hi how are you")
        }
    }

    //^ Now in main we can do this
    //^ Returning types that implement certain traits is very useful inside of closures and iterators, we will learn about closures and iterators in the far future
    fn main() {
        println!("{}", returns_summarizable().summarize()); //. We know we can call .summarize() because whatever returns_summarizable() returns will implement Summary
    }

    //^ Now lets look at how we can use Trait bounds to conditionally implement methods. Refer to c_methods

}

mod c_methods {
    use std::fmt::Display;

    //. Here we have a struct called Pair and it has two fields x and y both of which are of the type T
    struct Pair<T> {
        x: T,
        y: T,
    }

    //^ Then we have two implementation blocks the first implementation block is on the type Pair of T, Pair with a type parameter that is Generic 
    //. So it is basically saying that this implementation block is for any pair struct
    impl<T> Pair<T> {
        //. Here we have a new associated function that creates a new pair
        fn new(x: T, y: T) -> Self {
            Self {
                x, 
                y,
            }
        }
    }

    //^ Our second implementation block has impl<T: Display + PartialOrd> that basically says that the T of Pair<T> has to implement Display and PartialOrd
    impl<T: Display + PartialOrd> Pair<T> {
        //^ Next we define this compare Display method, it compares x and y then displays the Result. 
        //^ We can do this since we know that T will be a type that can be compared and displayed
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    //^ So every instance of Pair will have a new Associative function, but the compare display method will only be available to instances of Pair where the type of x and y can be displayed and compared
    //^ The last thing I want to talk about very briefly is Blanket Implementations, refer to b_implementations
}

mod b_implementations {
    use std::fmt::Display;

    //. Basically we can implement a trait on a type that implements another trait
    //^ Any type that can be displayed can be converted to a string
    //. Example
    trait ConvertToString {
        fn my_to_string(&self) -> String;
    }
    //. We fist check if T implements the display trait, if it does we implement the ConvertToString trait for T
    impl<T: Display> ConvertToString for T {
        fn my_to_string(&self) -> String {
            format!("{self}")
        }
    }

    //^ We will look more at Blanked Traits in our Advanced traits in Rust chapter, you probably wont use Blanked traits a lot, but I just want you to know what they are. Rusts Standard lib uses them a lot
}
fn main() {}
