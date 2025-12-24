#![allow(warnings)] //. Stops the compiler from generating warnings

//^ Collections allow you to store multiple values in one variable. But unlike arrays and tuples collections are stored on the Heap and not the stack. Meaning the size of the collection can grow or shrink as needed. 

//^ We will be talking about
//^ 1. Vectors
//^ 2. Strings
//^ 3. HashMaps


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
    
}
