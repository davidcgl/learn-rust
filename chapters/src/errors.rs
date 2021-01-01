use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn run() {
    // Unrecoverable Errors with panic!
    // This causes the program to unwind and quit.
    // set panic=abort in Cargo.toml to quit without unwinding.
    // panic!("crash!");
    // Recoverable Errors with Result
    let f1 = File::open("hello.txt");

    let f1 = match f1 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // panic! if open() returns an Error
    // let f2 = File::open("hello.txt").unwrap();

    // panic! if open() returns an Error. Like unwrap() but with a custom message
    // let f2 = File::open("hello.txt").expect("File not found");
}

// The verbose way
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The idiomatic way
fn read_username_from_file_2() -> Result<String, io::Error> {
    // The ? placed after a Result value is defined to work in almost the same way as the match
    // expressions we defined to handle the Result values in read_username_from_file_1. If the value
    // of the Result is an Ok, the value inside the Ok will get returned from this expression, and
    // the program will continue. If the value is an Err, the Err will be returned from the whole
    // function as if we had used the return keyword so the error value gets propagated to the
    // calling code.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// The idiomatic way with ? chaining
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// The most idiomatic way: use fs::read_to_string!
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
