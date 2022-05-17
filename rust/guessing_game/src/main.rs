use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("the secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // shadowing let's ur reuse a value
        // often used when we want to convert a value from one type to another type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small, get bigger!"),
            Ordering::Greater => println!("get smaller, giant"),
            Ordering::Equal => {
                println!("you guessed it, about time!");
                break;
            }
        }
    }
}
