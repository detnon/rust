fn main() {
    let number = 5;

    // Control flow with if statements and conditions
    if number >= 5 {
        println!("Number is 5 or greater ")
    } else {
        println!("Number less than 5")
    }

    let condition = true;

    let number = if condition { 3 } else { 4 };

    loopy();
    conditional_loop();
    iterate_over_collection();
}

fn loopy() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result)
}

fn conditional_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!");
}

fn iterate_over_collection() {
    let arr = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("index is {}", arr[index]);

        index += 1;
    }
}
