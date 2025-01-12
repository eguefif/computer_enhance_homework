pub mod parser;
use crate::parser::parser::parse_tokens;
use crate::parser::tokenizer::tokenize;
use std::collections::HashMap;

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
