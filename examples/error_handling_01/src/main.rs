use std::fs::File;
use std::io::ErrorKind;

fn panic_match() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) =>  file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match  File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => panic!("problem opening the file: {:?}", other_error)
        },
    };
}

fn panic_unwrap(){
    let f = File::open("goodbye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("goodbye.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem creating the file: {:?}", error);
        }
    });
}

fn panic_unwrap_2(){
    let f = File::open("hello.txt").unwrap();
}


fn panic_expect(){
    // .expect() is used to pass clear error messages into the panic!() macro
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn main() {
    panic_match();
    panic_unwrap();
}
