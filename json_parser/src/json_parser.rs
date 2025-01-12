#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::json_parser::parser::parse_tokens;
use crate::json_parser::tokenizer::tokenize;
use std::collections::HashMap;

pub mod parser;
pub mod tokenizer;

#[derive(Clone, Debug)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    List(Vec<Value>),
    Object(HashMap<String, Box<Value>>),
    Null,
}

pub fn json_parse(content: String) -> HashMap<String, Box<Value>> {
    let tokens = tokenize(&content);
    parse_tokens(tokens)
}
