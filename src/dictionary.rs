//! A [Dictionary] trait and [EnglishDictionary] which uses `/usr/share/dict/words` as source.
//!
use anyhow::Result;
use indexmap::IndexSet;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;

const DICTIONARY_PATH: &str = "/usr/share/dict/words";

/// Dictionary trait for online(not implemented) and offline implementations, and testing support.
pub trait Dictionary {
    /// Get a random word from the Dictionary.
    fn random_word(&self) -> &str;
    /// Check if word is present in the Dictionary.
    fn is_valid_word(&self, word: &str) -> bool;
}

/// Implements [Dictionary] using `/usr/share/dict/words` as source.
#[derive(Debug)]
pub struct EnglishDictionary {
    words: IndexSet<String>,
    rng_refcell: RefCell<ThreadRng>,
}

impl EnglishDictionary {
    /// Create a new English Dictionary of 5 letter words from
    /// contents of `/usr/share/dict/words`.
    ///
    /// It stores `thread_rng` to support `random_word` trait method.
    ///
    /// Falliable method as source file may not exist or the encoding is not utf8.
    pub fn new() -> Result<EnglishDictionary> {
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
}

impl Dictionary for EnglishDictionary {
    fn random_word(&self) -> &str {
        let random_index = self.rng_refcell.borrow_mut().gen_range(0..self.words.len());
        self.words.get_index(random_index).unwrap().as_str()
    }

    fn is_valid_word(&self, word: &str) -> bool {
        matches!(self.words.get(word), Some(_))
    }
}
