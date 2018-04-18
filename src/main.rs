extern crate names;
extern crate ws;

use std::collections::HashSet;
use names::{Generator, Name};
use ws::{listen, Handler, Sender, Result, Message, CloseCode};

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

struct Server {
    out: Sender,
    game: Game,
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let string_msg = msg.to_string();

        let res = if self.game.guesses.insert(string_msg.clone()) {
            check_letter(&mut self.game, &string_msg);
            format!("word: {:?}, misses: {:?}", self.game.progress, self.game.misses)
        } else {
            format!("{} has already been guessed", msg)
        };

        let res = if self.game.misses == 10 {
            self.game = start_game();
            format!("You lose, try again...")
        } else if self.game.progress == self.game.word {
            self.game = start_game();
            format!("You win, starting a new game!")
        } else { res };

        self.out.send(res)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

}

fn check_letter(game: &mut Game, guess: &String) {
    if game.word.contains(guess) {
        game.update_progress(guess);
    } else {
        game.increment_miss();
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
    listen("127.0.0.1:3000", |out| Server { out: out, game: start_game() } ).unwrap();
}
