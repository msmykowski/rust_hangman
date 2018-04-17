extern crate names;

use std::io;
use std::collections::HashSet;
use names::{Generator, Name};

#[derive(Debug)]
struct Game {
    word: Vec<String>,
    guesses: HashSet<String>,
    misses: u32,
    progress: Vec<String>,
}

impl Game {
    fn increment_miss(&mut self) {
        self.misses += 1;
    }

    fn check_letter(&mut self, guess: &String) {
        if self.word.contains(guess) {
            for (i, letter) in self.word.iter().enumerate() {
                if letter == guess {
                    self.progress[i] = letter.clone();
                }
            }
        } else {
            self.increment_miss();
        }
    }

    fn check_status(&self) -> Option<&str> {
        if self.misses == 10 {
            return Some("You lose");
        } else if self.progress == self.word {
            return Some("You win");
        }

        None
    }
}

fn generate_word() -> Vec<String> {
    let mut generator = Generator::with_naming(Name::Plain);
    generator.next()
             .unwrap()
             .split("")
             .map(|c| c.to_string())
             .filter(|s| s != "")
             .collect::<Vec<String>>()

}

fn start_game() -> Game {
    let word = generate_word();

    Game {
        progress: vec!["".to_string(); word.len()],
        word: word,
        guesses: HashSet::new(),
        misses: 0,
      }
}

fn main() {
    let mut game = start_game();

    loop {
        println!("Guess a letter");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();

        let guess = guess.trim()
                         .to_string();

        if game.guesses.insert(guess.clone()) {
            game.check_letter(&guess)
        } else {
            println!("{} has already been guessed", guess);
        }

        println!("{:?}", game.progress);
        println!("You missed {} times", game.misses);

        match game.check_status() {
            Some(x) => {
                println!("{}", x);
                break;
            },
            None    => continue,
        }
    }

}
