extern crate regex;

mod rule;
mod bound;

pub mod rules;

pub use rule::{Rule, ValidationResult, Error};
