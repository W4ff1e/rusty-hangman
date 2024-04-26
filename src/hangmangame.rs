pub struct HangmanGameState {
    pub phrase_to_guess: String,
    pub guessed_letters: Vec<char>,
    pub incorrect_guess_count: u32,
    pub game_over: bool,
    pub win: bool,
}
impl Default for HangmanGameState {
    fn default() -> Self {
        Self {
            phrase_to_guess: String::new(),
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            game_over: false,
            win: false,
        }
    }
}
impl HangmanGameState {
    pub fn new(phrase_to_guess: String) -> Self {
        HangmanGameState {
            phrase_to_guess,
            guessed_letters: Vec::new(),
            incorrect_guess_count: 0,
            game_over: false,
            win: false,
        }
    }

    #[allow(dead_code)]
    pub fn check_guess(&mut self, guess: char) -> bool {
        if self.phrase_to_guess.contains(guess) {
            if !self.guessed_letters.contains(&guess) {
                self.guessed_letters.push(guess);
            }
            true
        } else {
            self.incorrect_guess_count += 1;
            false
        }
    }
}
