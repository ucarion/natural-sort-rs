#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use natural_sort::natural_sort;

pub mod natural_sort;
