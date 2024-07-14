use std::io;
use std::collections::HashSet;

struct Game {
    word: String,
    attempts: u32,
    guessed_letters: HashSet<char>,
}

impl Game {
    fn new(word: &str) -> Self {
        Game {
            word: word.to_string(),
            attempts: 6,
            guessed_letters: HashSet::new(),
        }
    }

    fn display_word(&self) -> String {
        self.word.chars()
            .map(|c| if self.guessed_letters.contains(&c) { c } else { '_' })
            .collect()
    }

    fn make_guess(&mut self, guess: char) -> bool {
        if self.word.contains(guess) {
            self.guessed_letters.insert(guess);
            true
        } else {
            self.attempts -= 1;
            false
        }
    }

    fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.guessed_letters.contains(&c))
    }

    fn is_lost(&self) -> bool {
        self.attempts == 0
    }
}

fn main() {
    let word_to_guess = "rustacean";
    let mut game = Game::new(word_to_guess);

    println!("Welcome to Hangman!");

    while !game.is_won() && !game.is_lost() {
        println!("Word: {}", game.display_word());
        println!("Attempts left: {}", game.attempts);
        println!("Guessed letters: {:?}", game.guessed_letters);

        let mut guess = String::new();
        println!("Please enter your guess: ");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next().expect("Please enter a letter");

        if game.make_guess(guess) {
            println!("Good guess!");
        } else {
            println!("Wrong guess!");
        }
        println!();
    }

    if game.is_won() {
        println!("Congratulations, you won! The word was: {}", game.word);
    } else {
        println!("Game over, you lost! The word was: {}", game.word);
    }
}
