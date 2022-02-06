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

#[derive(Debug, PartialEq)]
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
    pub fn new(dictionary: &'w dyn Dictionary) -> Self {
        let word = dictionary.random_word().to_uppercase();

        if std::env::var("DEBUG").is_ok() {
            println!("[DEBUG] Word is {}", word);
        }

        Wordle {
            dictionary,
            word,
            current_attempt: Default::default(),
            guesses: Default::default(),
        }
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
            let mut input_letter_count = [0_u8; 26];
            for ch in self.word.as_bytes() {
                input_letter_count[(*ch - b'A') as usize] += 1
            }

            let mut processed: Vec<i8> = vec![1, 2, 3, 4, 5];
            let turn_input = &mut self.guesses[current_attempt];

            // first process exact matches
            for (idx, ch) in word.as_bytes().iter().enumerate() {
                turn_input[idx].chr = *ch;
                if self.word.as_bytes()[idx] == *ch {
                    turn_input[idx].mch = Match::ExactLocation;
                    input_letter_count[(ch - b'A') as usize] -= 1;
                    processed[idx] = -processed[idx];
                }
            }

            // process remaining letters (not present in word, or present in word)
            for position in processed.iter() {
                if *position > 0_i8 {
                    let index = (*position - 1) as usize;
                    let input_ch = word.as_bytes().get(index).unwrap();
                    let index_in_count = (*input_ch - b'A') as usize;
                    if input_letter_count[index_in_count] > 0 {
                        turn_input[index].mch = Match::PresentInWord;
                        input_letter_count[index_in_count] -= 1;
                    }
                }
            }

            if word == self.word {
                return Ok(PlayResult::YouWon(&self.guesses[current_attempt]));
            }

            if self.current_attempt == 6 {
                return Ok(PlayResult::YouLost(self.word.as_str()));
            } else {
                return Ok(PlayResult::TurnResult(&self.guesses[current_attempt]));
            }
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
                write!(
                    f,
                    "{:3}",
                    RGB(0, 0, 0)
                        .bold()
                        .on(RGB(255, 255, 0) /* Custom Yellow */)
                        .paint(letter)
                )?
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

mod tests {
    use super::*;

    struct TestDict;
    impl Dictionary for TestDict {
        fn random_word(&self) -> &str {
            "ARIEL"
        }

        fn is_valid_word(&self, word: &str) -> bool {
            ["ARIEL"].contains(&word)
        }
    }
    #[test]
    fn test_win_single_attempt() {
        let test_dict = TestDict {};
        let mut wordle = Wordle::new(&test_dict);
        let play_result = wordle.play("ArIeL");
        assert!(play_result.is_ok());
        let play_result = play_result.unwrap();

        let expected_turn_input = [
            Input {
                chr: b'A',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'R',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'I',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'E',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'L',
                mch: Match::ExactLocation,
            },
        ];

        match play_result {
            PlayResult::YouWon(computed) => {
                assert_eq!(computed.len(), expected_turn_input.len());
                assert!(computed
                    .iter()
                    .zip(expected_turn_input.iter())
                    .all(|(com, exp)| com.chr == exp.chr && com.mch == exp.mch))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_duplicate() {
        struct DupDict;
        impl Dictionary for DupDict {
            fn random_word(&self) -> &str {
                "GREED"
            }

            fn is_valid_word(&self, word: &str) -> bool {
                ["GREED", "ELITE"].contains(&word)
            }
        }

        let dup_dict = DupDict {};
        let mut wordle = Wordle::new(&dup_dict);
        let play_result = wordle.play("ELITE");
        assert!(play_result.is_ok());
        let play_result = play_result.unwrap();

        let expected_turn_input = [
            Input {
                chr: b'E',
                mch: Match::PresentInWord,
            },
            Input {
                chr: b'L',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'I',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'T',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'E',
                mch: Match::PresentInWord,
            },
        ];

        match play_result {
            PlayResult::TurnResult(computed) => {
                assert_eq!(computed.len(), expected_turn_input.len());
                assert!(computed
                    .iter()
                    .zip(expected_turn_input.iter())
                    .all(|(com, exp)| com.chr == exp.chr && com.mch == exp.mch))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_double_letters() {
        struct DupDict;
        impl Dictionary for DupDict {
            fn random_word(&self) -> &str {
                "GLIDE"
            }

            fn is_valid_word(&self, word: &str) -> bool {
                ["GLIDE", "GREED"].contains(&word)
            }
        }

        let dup_dict = DupDict {};
        let mut wordle = Wordle::new(&dup_dict);
        let play_result = wordle.play("GREED");
        assert!(play_result.is_ok());
        let play_result = play_result.unwrap();

        let expected_turn_input = [
            Input {
                chr: b'G',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'R',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'E',
                mch: Match::PresentInWord,
            },
            Input {
                chr: b'E',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'D',
                mch: Match::PresentInWord,
            },
        ];

        match play_result {
            PlayResult::TurnResult(computed) => {
                assert_eq!(computed.len(), expected_turn_input.len());
                assert!(computed
                    .iter()
                    .zip(expected_turn_input.iter())
                    .all(|(com, exp)| com.chr == exp.chr && com.mch == exp.mch))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_double_letters_input_match_in_future() {
        struct DupDict;
        impl Dictionary for DupDict {
            fn random_word(&self) -> &str {
                "TRULY"
            }

            fn is_valid_word(&self, word: &str) -> bool {
                ["TRULY", "KELLY"].contains(&word)
            }
        }

        let dup_dict = DupDict {};
        let mut wordle = Wordle::new(&dup_dict);
        let play_result = wordle.play("KELLY");
        assert!(play_result.is_ok());
        let play_result = play_result.unwrap();

        let expected_turn_input = [
            Input {
                chr: b'K',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'E',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'L',
                mch: Match::AbsentInWord,
            },
            Input {
                chr: b'L',
                mch: Match::ExactLocation,
            },
            Input {
                chr: b'Y',
                mch: Match::ExactLocation,
            },
        ];

        match play_result {
            PlayResult::TurnResult(computed) => {
                assert_eq!(computed.len(), expected_turn_input.len());
                assert!(computed
                    .iter()
                    .zip(expected_turn_input.iter())
                    .all(|(com, exp)| com.chr == exp.chr && com.mch == exp.mch))
            }
            _ => panic!(),
        }
    }
}
