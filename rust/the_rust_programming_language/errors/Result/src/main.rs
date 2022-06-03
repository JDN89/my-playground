use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
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


    // cleaner way or writing without all the matching
    use std::fs::File;
use std::io::ErrorKind;
// if the variant of Result is Ok -> unwrap will give us the value of Ok variant
// else -> unwrap calls the panic! macro

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    // expect does the same AND let's us chose the error message
    // als we unwrap blijven gebruiken is het moeilijker om de locatie van de bug  te vinden in een grotere code base
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

// propagating errors -> pass error info back to caller function
}
