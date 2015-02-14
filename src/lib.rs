//! `natural_sort` is a crate that deals with sorting strings using "natural
//! sorting", (also known as "human sorting").
//!
//! With normal string-based searching, strings are always compared
//! alphabetically:
//!
//! ```
//! let mut files = ["file2.txt", "file11.txt", "file1.txt"];
//! files.sort();
//!
//! // "file11.txt" comes before "file2.txt" because the string "11" comes
//! // before the string "2" in the dictionary.
//! assert_eq!(files, ["file1.txt", "file11.txt", "file2.txt"]);
//! ```
//!
//! This crate provides a function `natural_sort` which will order strings
//! numerically when doing so makes sense:
//!
//! ```
//! use natural_sort::natural_sort;
//!
//! let mut files = ["file1.txt", "file11.txt", "file2.txt"];
//! natural_sort(&mut files);
//!
//! // Here, "file11.txt" comes last because `natural_sort` saw that there was a
//! // number inside the string, and did a numerical, rather than lexical,
//! // comparison.
//! assert_eq!(files, ["file1.txt", "file2.txt", "file11.txt"]);
//! ```
//!
//! Human-comparable strings can be created directly using
//! `natural_sort::HumanString::from_str`.

#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

pub use natural_sort::natural_sort;
pub use natural_sort::HumanString;

pub mod natural_sort;
