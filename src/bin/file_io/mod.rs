use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn open() {    
    let path = Path::new("c:/dev/rust/text/huck.txt");
    let display = path.display();

    if !path.exists() {
        panic!("path doesnt exist {}", display)
    }

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };
    

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("\n{} contains:\n\n{}", display, s),
    }
}