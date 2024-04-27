use include_dir::{include_dir, Dir};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;

pub struct HangmanGameState {
    pub phrase_to_guess: String,
    pub chars_to_guess: Vec<char>,
    pub guessed_letters: Vec<char>,
    pub incorrect_guess_count: u32,
    pub difficulty: u32,
    pub game_over: bool,
    pub win: bool,
}
impl Default for HangmanGameState {
    fn default() -> Self {
        Self {
            phrase_to_guess: String::new(),
            chars_to_guess: Vec::new(),
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            difficulty: 0, // 6 us "normal difficulty" 4 is "hard difficulty" 8 is "easy difficulty", and 10 is "very easy difficulty"
            game_over: false,
            win: false,
        }
    }
}
impl HangmanGameState {
    pub fn new(phrase_to_guess: String) -> Self {
        HangmanGameState {
            phrase_to_guess: phrase_to_guess.clone(),
            chars_to_guess: phrase_to_guess.chars().collect(),
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            difficulty: 0,
            game_over: false,
            win: false,
        }
    }

    pub fn random_word_from_file(&mut self, length: u32) -> Result<String, io::Error> {
        let file_contents = include_str!("../worldlist/wordlist.txt");
        let lines = file_contents.lines();

        let mut words = Vec::new();

        for line in lines {
            if line.len() == length as usize {
                words.push(line.to_string());
            }
        }

        if words.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No words found with the specified length",
            ));
        }

        let random_word = words[rand::thread_rng().gen_range(0..words.len())].clone();

        Ok(random_word)
    }

    pub fn random_phrase_to_guess(&mut self, length: u32) {
        let phrase = self
            .random_word_from_file(length)
            .expect("Error getting random word from file");
        let phrase = phrase.to_uppercase();
        self.chars_to_guess = phrase.chars().collect();
        self.phrase_to_guess = phrase;
    }

    /// Update the guess phrase in the hangman game.
    ///
    /// # Arguments
    ///
    /// * `guess` - The new phrase to guess.
    ///
    #[allow(dead_code)]
    pub fn update_guess_phrase(&mut self, phrase: String) {
        let phrase = phrase.to_uppercase();
        self.chars_to_guess = phrase.chars().collect();
        self.phrase_to_guess = phrase;
    }

    #[allow(dead_code)]
    /// Guess a letter in the hangman game.
    ///
    /// # Arguments
    ///
    /// * `guess` - The letter to guess.
    ///
    pub fn guess_letter(&mut self, guess: char) {
        // Set the default difficulty if it is not already set
        if self.difficulty == 0 {
            self.difficulty = 6;
        }

        // Generate a random phrase if no phrase is set
        if self.phrase_to_guess == "" {
            self.random_phrase_to_guess(self.difficulty);
        }

        // Add the guessed letter to the list of guessed letters
        self.guessed_letters.push(guess);

        // Check if the guessed letter is in the phrase
        if self.chars_to_guess.contains(&guess) {
            // Check if all the characters in the phrase have been guessed
            if self
                .chars_to_guess
                .iter()
                .all(|c| self.guessed_letters.contains(c))
            {
                self.win = true;
                self.game_over = true;
            }
        } else {
            // Increment the incorrect guess count
            self.incorrect_guess_count += 1;

            // Check if the maximum incorrect guess count has been reached
            if self.incorrect_guess_count >= self.difficulty {
                self.game_over = true;
            }
        }
    }
}
