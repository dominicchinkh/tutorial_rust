use std::error::Error;
use std::fs::{File};
use std::io::{self, ErrorKind, Read};

// Custom Types for Validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// `main` can also return a `Result<(), E>`
// `Box<dyn Error>` type is a trait object
fn main() -> Result<(), Box<dyn Error>> {

    /*
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
    */

    // Another cleaner alternative:

    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    //------------------------------
    // Shortcuts for Panic on Error

    // If the Result value is the `Ok` variant, unwrap will return the value inside the `Ok`. 
    // If the Result is the `Err` variant, unwrap will call the `panic!` macro
    let _greeting_file = File::open("hello.txt").unwrap();

    // Similar to `wrap`, except the `expect` method lets us also choose the `panic!` error
    // message
    let _greeting_file = File::open("hello.txt")
                            .expect("hello.txt should be included in this project");

    //--------------------
    // Propagating Errors

    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}   

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {

    //-------------------------
    // Using match

    /*
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),

        // Note: we don’t need to explicitly say return, because this is the last expression
        //       in the function.
        Err(e) => Err(e),
    }
    */

    //-------------------------
    // The ? Operator Shortcut

    // If the value of the Result is an `Ok`, the value inside the `Ok` will get returned
    // from this expression, and the program will continue. If the value is an `Err`, the 
    // `Err` will be returned from the whole function as if we had used the return keyword 
    // so that the error value gets propagated to the calling code

    // Note that you can use the `?` operator on a `Result` in a function that returns 
    // `Result`, and you can use the `?` operator on an `Option` in a function that returns
    // `Option`, but you can’t mix and match.

    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    //-------------------------
    // Alternatively, with fs

    // fs::read_to_string("hello.txt")
}
