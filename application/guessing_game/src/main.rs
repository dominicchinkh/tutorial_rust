use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // `rand::thread_rng` function that gives us the particular random number 
    // generator we’re going to use: one that is local to the current thread of 
    // execution and is seeded by the operating system

    // `gen_range` method takes a range expression as an argument and generates a 
    // random unsigned 32-bit number in the range. 

    // `start..=end` is inclusive on the lower and upper bounds

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            // `read_line` takes whatever the user types into standard input and append 
            // that into a string (without overwriting its contents)
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string to a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catch-all value
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // The {} set of curly brackets is a placeholder
        // println!("x = {x} and y + 2 = {}", y + 2);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
