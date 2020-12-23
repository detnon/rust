struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

// let user1 = User {
//    email: String::from("testname@test.com"),
//    username: String ::from("testuser"),
//    active: true,
//    sign_in_count: 1,
//}

//user1.email = String::from("different@email.com")


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
}
