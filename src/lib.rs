#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! A library for Wordle and Dictionary to use with Wordle.
//!
//! # Play
//!
//! ```bash
//! wordler
//! ```
//!
//! ![Play Demo](../../../play-demo.gif)
//!
//! # Example
//!
//! Basic usage:
//!
//! ```
//! use wordler::dictionary::EnglishDictionary;
//! use wordler::wordle::{Wordle, PlayResult};
//!
//! let dictionary = EnglishDictionary::new().unwrap();
//! let mut wordle = Wordle::new(&dictionary);
//! let play_result = wordle.play("dream");
//! match play_result {
//!   Ok(play_result) => {
//!     println!("{}", play_result);
//!     match play_result {
//!         PlayResult::YouWon(_) => std::process::exit(0),
//!         PlayResult::YouLost(_) => std::process::exit(1),
//!         PlayResult::TurnResult(_) => {}
//!     }
//!   }
//!   Err(e) => println!("{}", e),
//! }
//! ```

pub mod dictionary;
pub mod wordle;

// pub mod prelude {
//     //! Make import easy
//     pub use crate::wordle::*;
//     pub use crate::dictionary::*;
// }
