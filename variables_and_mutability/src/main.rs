fn main() {
    // Constants cannot be made mutable with `mut` and must be type annotated
    // Cannot be set as a result of a function.
    const OPINION: &str = "Rust is pretty cool.";
    println!("{}", OPINION);

    // variables must be declared as mutable, otherwise
    // will not be able to be set further on
    // eg let x = 5; cannot be set further along the code.

    let mut x = 5;

    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    second_function();

    // variables can be assigned with the return values of functions like so
    let f = third_function();
    let s = fourth_function();
    println!("{} is the return value!", f);
    println!("{} is the return value!", s)
}

// Functions are called within main and are named in `snake_case`
fn second_function() {
    println!("The second function");

    // Arrays.
    // We can declare an array like this, setting the type of
    // the elements and stating the number of elements
    let ar_one: [i32; 5] = [1, 2, 3, 4, 5];

    // We can set an element of the array to another variable and
    // print it out
    let aro = ar_one[0];
    println!("{}", aro);
}

// functions can return values either by declaring it last within a function
fn third_function() -> i32 {
    5
}
// or by the return statement
fn fourth_function() -> i32 {
    return 6;
}
