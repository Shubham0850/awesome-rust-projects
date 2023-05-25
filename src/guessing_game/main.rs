use rand::Rng; // Importing the Rng trait from the rand crate
use std::{cmp::Ordering, io}; // Importing the Ordering enum and io module from the standard library

fn main() {
    // Generate a random number between 1 and 100 (inclusive) and assign it to secret_number
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Print a message to prompt the user to guess the number
    println!("Guess the number!");

    loop {
        // Prompt the user to input their guess
        println!("Please input your guess.");

        // Create a mutable string variable to store the user's guess
        let mut guess: String = String::new();

        // Read the user's input from the console and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // Convert the user's input to an unsigned 32-bit integer (u32)
            // If parsing is successful, assign the parsed number to guess
            Ok(num) => num,
            // If an error occurs during parsing, skip the current iteration of the loop and prompt the user to input another guess
            Err(_) => continue,
        };

        // Print the user's guess
        println!("You guessed: {}", guess);

        // Compare the user's guess with the secret_number
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret_number, print "Too small!"
            Ordering::Less => println!("Too small!"),
            // If the guess is greater than the secret_number, print "Too big!"
            Ordering::Greater => println!("Too big!"),
            // If the guess is equal to the secret_number
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop
            }
        }
    }
}
