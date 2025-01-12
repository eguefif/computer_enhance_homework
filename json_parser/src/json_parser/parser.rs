use crate::json_parser::tokenizer::Token;
use crate::json_parser::tokenizer::Token::{
    Bool, BracketClose, BracketOpen, Comma, CurlyClose, CurlyOpen, Key, Null, Num, Str,
};
use crate::json_parser::Value;

use std::collections::HashMap;

pub fn parse_tokens(tokens: Vec<Token>) -> HashMap<String, Box<Value>> {
    let mut iterator = tokens.into_iter();
    get_json(&mut iterator)
}

fn get_json(iterator: &mut impl Iterator<Item = Token>) -> HashMap<String, Box<Value>> {
    let mut retval = HashMap::new();
    let mut key = "".to_string();
    let mut value = Box::new(Value::Null);
    loop {
        if let Some(token) = iterator.next() {
            match token {
                CurlyOpen => value = Box::new(Value::Object(get_json(iterator))),
                CurlyClose => break,
                BracketOpen => {}
                BracketClose => {}
                Comma => {
                    retval.insert(key.clone(), value);
                    key.clear();
                    value = Box::new(Value::Null);
                }
                Key(v) => key = v,
                Str(v) => value = Box::new(Value::Str(v)),
                Num(v) => value = Box::new(Value::Num(v)),
                Bool(v) => value = Box::new(Value::Bool(v)),
                Null => value = Box::new(Value::Null),
            }
        } else {
            break;
        }
    }
    retval
}
