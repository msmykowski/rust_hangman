extern crate names;
extern crate ws;

#[macro_use]
extern crate json;

use names::{Generator, Name};
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use ws::{listen, CloseCode, Handler, Message, Result, Sender};

trait Job : Send {
    fn execute(&self, game: &mut Game);
}

struct Request {
    out: Sender,
    msg: String,
}

impl Request {
    fn new(msg: String, out: Sender) -> Request {
        Request {
            msg: msg,
            out: out
        }
    }
}

impl Job for Request {
    fn execute(&self, mut game: &mut Game) {
        if game.guesses.insert(self.msg.clone()) {
            check_letter(&mut game, &self.msg);
        };

        let status = game.status();

        let progress = game.progress.clone();
        let guesses = game.guesses.clone().into_iter().collect::<Vec<String>>();
        println!(
            "guesses: {:?}, progress: {:?}, misses: {}, status: {}",
            guesses, progress, game.misses, status
        );

        self.out.broadcast(json::stringify(object!{
            "status"  => game.status(),
            "progress" => progress,
            "guesses" => guesses,
            "misses" => game.misses,
        })).unwrap();
    }
}

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

    fn status(&self) -> &str {
        if self.misses > 10 {
            "lose"
        } else if self.progress == self.word {
            "win"
        } else {
            "active"
        }
    }
}

struct Server {
    out: Sender,
    tx: std::sync::mpsc::Sender<Box<Job>>,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let string_msg = msg.to_string();
        self.tx
            .send(Box::new(Request::new(string_msg, self.out.clone())))
            .unwrap();

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
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
    generator
        .next()
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
    let (tx, rx) = mpsc::channel::<Box<Job>>();

    thread::spawn(move || {
        for mut received in rx {
            received.execute(&mut game);
        }
    });

    listen("127.0.0.1:3000", |out| Server {
        out: out,
        tx: mpsc::Sender::clone(&tx),
    }).unwrap();
}
