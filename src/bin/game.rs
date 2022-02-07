use std::process::exit;

use anyhow::Result;
use wordler::dictionary::EnglishDictionary;
use wordler::wordle::{PlayResult, Wordle};

fn main() -> Result<(), anyhow::Error> {
    let dictionary = EnglishDictionary::new()?;
    let mut wordle = Wordle::new(&dictionary);
    let mut current_guess = String::new();
    loop {
        current_guess.clear();
        println!("Enter your guess [{}/6]", wordle.current_attempt());
        std::io::stdin().read_line(&mut current_guess)?;
        let play_result = wordle.play(current_guess.trim());
        match play_result {
            Ok(play_result) => {
                println!("{}", play_result);
                match play_result {
                    PlayResult::YouWon(_) => exit(0),
                    PlayResult::YouLost(_, _) => exit(1),
                    _ => {}
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
