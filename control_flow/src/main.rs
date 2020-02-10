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
    while_over_collection();
    iterate_over_collection();
    for_in_range();
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

fn for_in_range() {
    for number in (1..4).rev() {
        println!("The numbers are:{}!", number)
    }
}

fn iterate_over_collection() {
    let col = [10, 20, 30, 40, 50];
    let mut index = 0;

    for element in col.iter() {
        println!("The value is: {}", element)
    }
}

fn while_over_collection() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("index is {}", arr[index]);

        index += 1;
    }
}
