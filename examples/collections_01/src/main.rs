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

// Iterate over Vectors
let vi = vec![100, 32, 57];

for i in &vi {
    println!("{}", i);
}

// When iterating over this vector it must be `mut`
// in order to change the values
let mut vm = vec![42, 26, 9];
for i in &mut vm {
    // `*` dereferences the value of `i` in order to change it
    *i += 50;
    println!("{}", i);
}

} // V will go out of scope after this point
