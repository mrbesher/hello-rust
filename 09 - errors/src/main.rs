#![allow(unused)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // calling `panic!` macro causes the program to unwind
    // and clean up the stack, this behavior can be modified
    // in `Cargo.toml` file as follows
    /*
    [profile.release]
    panic = 'abort'
    */
    // panic!("crash and burn");

    // `Result` allow us to handle recoverable errors
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("Cannot open file: {:?}", e),
    };

    // using closures would make for a cleaner code for
    // more complex matches. closures are discussed later
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // `unwrap` is a method that returns the value inside `Ok`
    // in case of success and calls `panic!` in case of failure
    let f = File::open("hello.txt").unwrap();

    // `expect` allows us to write an error msg
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// an example of a function returning result propagating
// the error
fn read_username_from_file_basic() -> Result<String, io::Error> {
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

// using the `?` operator as a shortcut for propagating
// errors would yield a more concise code
fn read_username_from_file() -> Result<String, io::Error> {
    // the `?` returns the content of Ok on success and the
    // error is returned from the whole function on failure
    let mut s = String::new();
    // note that `?` can only be used in functions
    // returning `Result`, `Option`
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
