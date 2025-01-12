#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::json_parser::parser::parse_tokens;
use crate::json_parser::tokenizer::tokenize;

pub mod parser;
pub mod tokenizer;

pub fn json_parse(content: String) -> Pairs {
    let tokens = tokenize(&content);
    parse_tokens(tokens)
}

#[derive(Debug)]
pub struct Pair {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

impl Pair {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64) -> Pair {
        Pair { x0, x1, y0, y1 }
    }
}

#[derive(Debug)]
pub struct Pairs {
    pairs: Vec<Pair>,
}

impl Pairs {
    pub fn new() -> Pairs {
        Pairs { pairs: vec![] }
    }
}
