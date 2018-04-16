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

    fn update_progress(&mut self, guess: &String) {
        for (i, letter) in self.word.iter().enumerate() {
            if letter == guess {
                self.progress[i] = letter.clone();
            }
        }
    }
}

fn start_game() -> Game {
    let mut generator = Generator::with_naming(Name::Plain);
    let word = generator.next().unwrap();
    let word = word.split("")
                   .map(|c| c.to_string())
                   .collect::<Vec<String>>();

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

        let guess = guess.trim().to_string();

        if game.guesses.insert(guess.clone()) {
            if game.word.contains(&guess) {
                game.update_progress(&guess);
            } else {
                game.increment_miss();
            }
        } else {
            println!("{} has already been guessed", guess);
        }

        println!("{:?}", game.progress);
        println!("You missed {} times", game.misses);

        if game.misses == 10 {
            println!("You lose");
            break;
        }

        if game.progress == game.word {
            println!("You win");
            break;
        }
    }

}
