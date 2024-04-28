use rand::Rng;
use std::io::{self};

/// Struct representing the state of the Hangman game.
pub struct HangmanGameState {
    /// The phrase to guess in the Hangman game.
    pub phrase_to_guess: String,
    // The phrase to guess but with the characters replaced with underscores.
    pub obfuscated_phrase: String,
    /// The characters to guess in the Hangman game.
    pub chars_to_guess: Vec<char>,
    /// The letters that have been guessed in the Hangman game.
    pub guessed_letters: Vec<char>,
    /// The count of incorrect guesses in the Hangman game.
    pub incorrect_guess_count: u32,
    /// The difficulty level of the Hangman game. 4 - 10 Hard, Normal, Easy, Very Easy
    pub difficulty: u32,
    /// Flag indicating if the game is over.
    pub game_over: bool,
    /// Flag indicating if the player has won the game.
    pub win: bool,
    // Debug Flag
    pub show_debug: bool,
}

/// Implement the Default trait for HangmanGameState.
impl Default for HangmanGameState {
    /// Create a new instance of HangmanGameState with default values.
    fn default() -> Self {
        Self {
            phrase_to_guess: String::new(),
            obfuscated_phrase: String::new(),
            chars_to_guess: Vec::new(),
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            difficulty: 0, // 6 is "normal difficulty", 4 is "hard difficulty", 8 is "easy difficulty", and 10 is "very easy difficulty"
            game_over: false,
            win: false,
            show_debug: false,
        }
    }
}

impl HangmanGameState {
    /// Create a new instance of HangmanGameState with the given phrase to guess.
    ///
    /// # Arguments
    ///
    /// * `phrase_to_guess` - The phrase to guess in the Hangman game.
    ///
    pub fn new(phrase_to_guess: String) -> Self {
        HangmanGameState {
            phrase_to_guess: phrase_to_guess.clone(),
            obfuscated_phrase: String::new(),
            chars_to_guess: phrase_to_guess.chars().collect(),
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            difficulty: 0,
            game_over: false,
            win: false,
            show_debug: false,
        }
    }

    /// Generate a random word from a file with the specified length.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the word to generate.
    ///
    /// # Returns
    ///
    /// * `Result<String, io::Error>` - The randomly generated word, or an error if no words with the specified length are found.
    pub fn random_word_from_file(&mut self, length: u32) -> Result<String, io::Error> {
        let file_contents = include_str!("../worldlist/wordlist.txt");
        let lines = file_contents.lines();

        let mut words = Vec::new();

        // Iterate over each line in the file
        for line in lines {
            // Check if the line has the specified length
            if line.len() == length as usize {
                words.push(line.to_string());
            }
        }

        // Check if no words with the specified length are found
        if words.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No words found with the specified length",
            ));
        }

        // Generate a random index within the range of the words vector
        let random_word = words[rand::thread_rng().gen_range(0..words.len())].clone();

        Ok(random_word)
    }

    pub fn obfuscate_phrase(&mut self) {
        let mut obfuscated_phrase = String::new();

        for c in self.chars_to_guess.iter() {
            if self.guessed_letters.contains(c) {
                obfuscated_phrase.push(*c);
                obfuscated_phrase.push(' ');
            } else if c.is_whitespace() {
                obfuscated_phrase.push(*c);
                obfuscated_phrase.push(' ');
            } else {
                obfuscated_phrase.push('_');
                obfuscated_phrase.push(' ');
            }
        }

        self.obfuscated_phrase = obfuscated_phrase;
    }

    /// Generate a random phrase to guess in the hangman game.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the phrase to generate.
    ///
    pub fn random_phrase_to_guess(&mut self, length: u32) {
        // Get a random word from the file with the specified length
        let phrase = self
            .random_word_from_file(length)
            .expect("Error getting random word from file");

        // Convert the phrase to uppercase
        let phrase = phrase.to_uppercase();

        // Update the characters to guess with the characters from the phrase
        self.chars_to_guess = phrase.chars().collect();

        // Update the phrase to guess with the generated phrase
        self.phrase_to_guess = phrase;
        // Reobfuscate the phrase
        self.obfuscate_phrase();
    }

    /// Update the guess phrase in the hangman game.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The new phrase to guess.
    ///
    #[allow(dead_code)]
    pub fn update_guess_phrase(&mut self, phrase: String) {
        let phrase = phrase.to_uppercase();
        self.chars_to_guess = phrase.chars().collect();
        self.phrase_to_guess = phrase;
        self.obfuscate_phrase();
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
        self.obfuscate_phrase();
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
