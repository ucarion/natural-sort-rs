#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub mod natural_sort;

#[test]
fn test() {
    assert!(true);
}
