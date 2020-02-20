fn main() {
    // Each value can have only one owner at a time.
    // When a value goes out of scope, it is dropped.
    scope_example_one();
    scope_example_two();
    scope_example_move();
    scope_example_clone();
}

fn scope_example_one() {
    // the string literal `s` is only available after is has been declared and
    // before the closing '}'
    let s = "hello";
    // this type of string cannot be mutated
    println!("{}", s)
}

fn scope_example_two() {
    let mut st = String::from("hello");
    // String::from requests the memory it needs from the heap.
    // With this implementation we the variable can grow as
    // its final size is unknown.
    st.push_str(", world!");

    println!("{}", st);
} // Scope is over and `st` is  no longer valid

fn scope_example_move() {
    let s1 = String::from("hello");
    // when s1 is assigned to s2 like this, the data is not copied,
    // but the pointer that references its place on the heap is.
    let s2 = s1;
    println!("{}", s2)
    // This will invalidate `s1` as when `s2` goes out of scope this will cause
    // a double memory error.
    // for example `println!("{}, s1")` will not work at this point.
    // working this way is inexpensive in terms of memory usage
}

fn scope_example_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // calling the .clone() function will actually copy the heap data
    println!("s1 = {}, s2 = {}", s1, s2);
    // this is more memory expensive
}
