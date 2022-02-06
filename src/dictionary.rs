use anyhow::Result;
use indexmap::IndexSet;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;

const DICTIONARY_PATH: &str = "/usr/share/dict/words";

#[derive(Debug)]
pub struct EnglishDictionary {
    words: IndexSet<String>,
    rng_refcell: RefCell<ThreadRng>,
}

impl EnglishDictionary {
    pub fn load() -> Result<Self> {
        let contents = std::fs::read(DICTIONARY_PATH)?;
        let contents = String::from_utf8(contents)?;
        let words: IndexSet<String> = contents
            .split_whitespace()
            .filter(|w| w.len() == 5)
            .map(|w| w.to_string().to_uppercase())
            .collect();

        Ok(EnglishDictionary {
            words,
            rng_refcell: RefCell::new(rand::thread_rng()),
        })
    }

    pub fn random_word(&self) -> &str {
        let random_index = self.rng_refcell.borrow_mut().gen_range(0..self.words.len());
        self.words.get_index(random_index).unwrap().as_str()
    }

    pub fn is_valid_word(&self, word: &str) -> bool {
        matches!(self.words.get(word), Some(_))
    }
}
