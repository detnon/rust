use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favourite Colour");
    let field_value = String::from("Black");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Accessing values in a HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 40);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    // this is done by inserting a key value pair with the
    // key is what you want the value to be replaced
    
    println!("\nUpdating Red Scores\n");
    scores.insert(String::from("Red"), 1);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
