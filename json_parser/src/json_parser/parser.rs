use crate::json_parser::tokenizer::Token;
use crate::json_parser::tokenizer::Token::{
    Bool, BracketClose, BracketOpen, Colon, Comma, CurlyClose, CurlyOpen, Key, Null, Num, Str,
};
use crate::json_parser::Value;

use std::collections::HashMap;

pub fn parse_tokens(tokens: Vec<Token>) -> HashMap<String, Value> {
    let mut iterator = tokens.into_iter();
    get_json(iterator)
}

fn get_json(mut iterator: impl Iterator<Item = Token>) -> HashMap<String, Value> {
    let mut retval = HashMap::new();
    loop {
        if let Some(token) = iterator.next() {
            match token {
                CurlyOpen => {}
                CurlyClose => {}
                BracketOpen => {}
                BracketClose => {}
                Comma => {}
                Colon => {}
                Str(value) => {}
                Num(value) => {}
                Bool(value) => {}
                Key(value) => {}
                Null => {}
            }
        } else {
            break;
        }
    }
    retval
}
