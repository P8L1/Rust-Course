//^ Last week we started Traits, this week we will be finishing Traits by covering Lifetimes
//& In week 1 we did ownership and we covered dangling references, if we look back I said that a dangling reference can be fixed by using a lifetime.
//& Lets look at exactly that

mod lifetimes_1_dangling_references {
    //. Lets first look at a dangling reference example:
    fn main() {
        let r;
        {
            let x = 5;
            r = &x;
        } //. Since r is a reference to x, and x is dropped here r will be a reference to invalid data after this line
        println!("r: {}", r);
    }

    //. So a dangling reference is any reference that points to invalid data
    //. What is a lifetime? In short a lifetime is a measurement of how long a value lives in memory; so a lifetime is how long a variable lives before being dropped

    //. Lets look at this example where I annotated the lifetimes for you
    fn some_function() {
        let r;                 //. . --------------------- + ----  ' a
                                           //.                                       |
        {                                 //. ------ + ---- ' b             |
            let x = 5;        //.            |                           |
            r = &x;                   //.            |                           |
        }                                 // . ----- -                           |                     
        println!("r: {}", r);      //. ---------------------- +
    }

    //& So r has a lifetime of 'a (tick a)
    //& x has a lifetime of 'b  (tick b)

    //. Now lets actually take a look at how we can use lifetimes, refer to generic_lifetimes_1
}

mod generic_lifetimes_1 {
    //. Lets first look at an example program where we can use lifetimes
    //. We want a function called `longest`:
    //. - It takes two references to string slices (&str)
    //. - It returns a reference to the longer slice

    //. IMPORTANT: `longest` returns a REFERENCE, not an owned String.
    //. So Rust must guarantee the returned reference never outlives the data it points to.

    //. The tricky part:
    //. - The function might return `x`
    //. - OR it might return `y`
    //. So the compiler needs proof of how the output reference lifetime relates to BOTH inputs.

    //. Here is the function (this version is missing explicit lifetimes, so Rust can’t prove safety):
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    //. Why is this unsafe / ambiguous?
    //. Because we could call it like this:

    fn main() {
        let result: &str;                 //. result is a reference, so it must point to valid data
        let a = String::from("Andre");    //. a lives for the whole main()

        {
            let b:  String = String::from("JP");   //. b lives ONLY inside this inner scope

            result = longest(&a, &b);     //. could return &a OR could return &b
        }                                 //. b is dropped here

        //. If `longest` returned &b, then result would now be pointing at freed memory (DANGLING)
        //. Rust prevents this by forcing us to describe the lifetime relationship properly.
        println!("The longest string is {}", result);
    }

    //. The fix is lifetime annotations:
    //. We tell Rust: "the returned reference is valid for the SAME lifetime 'a as both inputs"
    //. That means the result can only be used as long as BOTH x and y are still alive.
    //
    //. In practice: the result’s lifetime becomes the SHORTER of the two input lifetimes,
    //. because that’s the longest time we can guarantee BOTH are valid.

    fn longest_fixed<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}








fn main() {
    println!("Hello, world!");
}
