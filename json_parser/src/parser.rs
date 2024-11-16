#![allow(dead_code)]

use crate::parser::tokenizer::tokenize;
pub mod tokenizer;

pub fn json_parse(content: String) -> Pairs {
    let tokens = tokenize(content);
    parse(tokens)
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

impl Default for Pairs {
    fn default() -> Self {
        Self::new()
    }
}

const OPENINGS: [char; 4] = ['{', '[', '"', ':'];
const CLOSINGS: [char; 4] = ['}', '}', '"', ','];

fn parse(tokens: Vec<String>) -> Pairs {
    Pairs::new()
}
