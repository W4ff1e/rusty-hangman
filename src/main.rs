use std::{char, io};

fn main() {
    println!("Welcome to Hangman!");

    loop {
        println!("Please enter your phrase: ");

        // Declare an empty string for the buffer to fill.
        let mut phrase_to_guess: String = String::new();

        //  Read terminal input
        io::stdin()
            .read_line(&mut phrase_to_guess)
            .expect("Failed to read terminal input!");

        // Make phrase to guess immutable for this iteration.
        let phrase_to_guess = phrase_to_guess;

        loop {
            println!("Please enter a letter to guess:");
            let mut letter_to_guess: String = String::new();

            io::stdin()
                .read_line(&mut letter_to_guess)
                .expect("Failed to read line!");

            let letter_to_guess: char = match letter_to_guess.trim().parse() {
                Ok(char) => char,
                Err(_) => continue,
            };

            for character in 0..phrase_to_guess.chars().count() {
                if letter_to_guess == phrase_to_guess.chars().nth(character).unwrap() {
                    println!("Match!")
                } else {
                    println!("No Match!");
                }
            }
        }
    }
}
