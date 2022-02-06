use crate::dictionary::Dictionary;
use ansi_term::Color::{Green, Red, White, RGB};
use anyhow::Result;
use std::fmt::Display;

pub struct Wordle<'w> {
    dictionary: &'w dyn Dictionary,
    word: String,
    current_attempt: u8,
    guesses: [TurnInput; 6],
}

//type Distribution = HashMap<u8, u8>;

#[derive(Debug)]
pub enum Match {
    ExactLocation,
    PresentInWord,
    AbsentInWord,
}

impl Default for Match {
    fn default() -> Self {
        Match::AbsentInWord
    }
}

#[derive(Debug, Default)]
pub struct Input {
    chr: u8,
    mch: Match,
}

pub type TurnInput = [Input; 5];

pub enum PlayResult<'w> {
    TurnResult(&'w TurnInput),
    YouWon(&'w TurnInput),
    YouLost(&'w str),
}

impl<'w> Wordle<'w> {
    pub fn new(dictionary: &'w dyn Dictionary) -> Result<Self> {
        let word = dictionary.random_word().to_uppercase();

        if std::env::var("DEBUG").is_ok() {
            println!("[DEBUG] Word is {}", word);
        }

        Ok(Wordle {
            dictionary,
            word,
            current_attempt: Default::default(),
            guesses: Default::default(),
        })
    }

    pub fn current_attempt(&self) -> u8 {
        self.current_attempt + 1
    }

    pub fn play(&mut self, word: &str) -> Result<PlayResult> {
        if self.current_attempt == 6 {
            return Ok(PlayResult::YouLost(&self.word));
        }

        if word.len() > 5 {
            return Err(anyhow::anyhow!(
                "Please enter a valid word with 5 letters. Word too long."
            ));
        }

        if word.len() < 5 {
            return Err(anyhow::anyhow!(
                "Please enter a valid word with 5 letters. Word too short."
            ));
        }

        let word = word.to_uppercase();
        if self.dictionary.is_valid_word(word.as_str()) {
            let current_attempt = self.current_attempt as usize;
            self.current_attempt += 1;
            for (idx, ch) in word.as_bytes().iter().enumerate() {
                let turn = &mut self.guesses[current_attempt];
                turn[idx].chr = *ch;
                if self.word.as_bytes()[idx] == *ch {
                    turn[idx].mch = Match::ExactLocation
                } else {
                    match self
                        .word
                        .contains(std::str::from_utf8([*ch].as_slice()).unwrap())
                    {
                        true => turn[idx].mch = Match::PresentInWord,
                        false => turn[idx].mch = Match::AbsentInWord,
                    }
                }
            }

            return match self.word == word {
                true => Ok(PlayResult::YouWon(&self.guesses[current_attempt])),
                false => {
                    if self.current_attempt == 6 {
                        Ok(PlayResult::YouLost(&self.word))
                    } else {
                        Ok(PlayResult::TurnResult(&self.guesses[current_attempt]))
                    }
                }
            };
        }

        Err(anyhow::anyhow!(
            "Please enter a valid word with 5 letters. Word not in dictionary: {}",
            word
        ))
    }
}

fn fmt_turn_input(f: &mut std::fmt::Formatter<'_>, turn_input: &TurnInput) -> std::fmt::Result {
    for input in turn_input {
        let letters = [b' ', input.chr, b' '];
        let letter = std::str::from_utf8(letters.as_slice()).unwrap();
        match input.mch {
            Match::AbsentInWord => write!(f, "{:3}", White.bold().on(Red).paint(letter))?,
            Match::ExactLocation => write!(f, "{:3}", RGB(0, 0, 0).bold().on(Green).paint(letter))?,
            Match::PresentInWord => {
                write!(f, "{:3}", RGB(0, 0, 0).bold().on(RGB(255,255,0) /* Custom Yellow */).paint(letter))?
            }
        }
    }
    Ok(())
}

impl<'w> Display for PlayResult<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PlayResult::TurnResult(turn_input) => fmt_turn_input(f, turn_input),
            PlayResult::YouLost(word) => writeln!(f, "You lost! The word is {}", word),
            PlayResult::YouWon(turn_input) => {
                fmt_turn_input(f, turn_input)?;
                writeln!(f, "\nCongratulations you won!")
            }
        }
    }
}
