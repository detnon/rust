fn main() {
// Vectors can be created empty with the type annotated
let v: Vec<i32> = Vec::new(); 

// The type can also be inferred by rust when using the `vec!` macro
let mut v = vec![1, 2, 3];

// New data can be pushed to a vector like so:

v.push(4);
v.push(5);
v.push(6);

// Vectors can only contain one type
} // V will go out of scope after this point
