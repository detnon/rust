fn main() {
    // Creating srings
    let mut s = String::new();

    // Strings can be created like this
    let data = "This is one helluv'a string!";
    let s = data.to_string();

    println!("{}",s);

    // Or like this
    let mut s = String::from("This is also one helluv'a string!");
    println!("{}", s);

    // Strings can be updated
    s.push_str(" Look at this cheeky little addition!");
    println!("{}",s);
}
