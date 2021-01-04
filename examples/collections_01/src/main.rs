fn main() {
// Vectors can be created empty with the type annotated
let v: Vec<i32> = Vec::new(); 

// The type can also be inferred by rust when using the `vec!` macro
// Vectors can only contain one type

let mut v = vec![1, 2, 3];

// New data can be pushed to a vector like so:

v.push(4);
v.push(5);
v.push(6);

let nv = vec![1,2,3,4,5];

let third: &i32 = &nv[2];
println!("the third element is {}", third);

match nv.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}


} // V will go out of scope after this point
