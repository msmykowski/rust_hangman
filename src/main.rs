extern crate names;

use std::io;
use std::collections::HashSet;
use names::{Generator, Name};

#[derive(Debug)]
struct Game<'a> {
    word: Vec<&'a str>,
    guesses: HashSet<String>,
    misses: u32,
    progress: Vec<String>,
}

impl<'a> Game<'a> {
    fn increment_miss(&mut self) {
        self.misses += 1;
    }

    fn update_progress(&mut self, guess: &str) {
        for (i, letter) in self.word.iter().enumerate() {
            if letter == &guess {
                self.progress[i] = letter.to_string();
            }
        }
    }
}

fn main() {
    let mut generator = Generator::with_naming(Name::Plain);
    let word = generator.next().unwrap();
    let word = word.split("").collect::<Vec<&str>>();
    let len = word.len();

    let mut game = Game { word: word, guesses: HashSet::new(), misses: 0, progress: vec!["".to_string(); len] };

    loop {
        println!("Guess a letter");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();

        let guess = guess.trim();

        if game.guesses.insert(guess.to_string()) {
            if game.word.contains(&guess) {
                game.update_progress(guess);
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
