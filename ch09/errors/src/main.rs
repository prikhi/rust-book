use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            _ => panic!("there was a problem opening the file: {:?}", e),
        },
    };

    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    // throws panic on error case
    let _f = File::open("derp.txt").unwrap();
    // throws panic with custom error
    let _f = File::open("derpl.txt").expect("Failed to open derpl.txt");

    let _ = read_username_from_file();
    let _ = simpler_read_username_from_file();
    let _ = read_username_chain();

    use std::net::IpAddr;
    let _home: IpAddr = "127.0.0.1".parse().unwrap(); // safe to use unwrap here
}

use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
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

// `?` will unwrap Oks & cause Err case to be returned
fn simpler_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// chaining makes it even shorter
fn read_username_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("user.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// using types to validate parameters
pub struct Guess {
    value: i32,
}

impl Guess {
    // the book returns Guess, but even better would be Result<GuessError,Guess>
    // with no panic!
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
