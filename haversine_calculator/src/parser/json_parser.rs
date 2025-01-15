use crate::parser::tokenizer::Token;
use crate::parser::tokenizer::Token::{
    Bool, BracketClose, BracketOpen, Comma, CurlyClose, CurlyOpen, Key, Null, Num, Str,
};
use crate::parser::Value;

use std::collections::HashMap;

pub fn parse_tokens(tokens: Vec<Token>) -> HashMap<String, Box<Value>> {
    let mut iterator = tokens.into_iter();
    iterator.next();
    get_json(&mut iterator)
}

fn get_json(iterator: &mut impl Iterator<Item = Token>) -> HashMap<String, Box<Value>> {
    let mut retval = HashMap::new();
    let mut key: Option<String> = None;
    let mut value: Option<Value> = None;
    let mut list: Vec<Value> = Vec::new();
    loop {
        if let Some(token) = iterator.next() {
            match token {
                Key(v) => key = Some(v.to_string()),
                CurlyOpen => value = Some(Value::Object(get_json(iterator))),
                Str(v) => value = Some(Value::Str(v)),
                Num(v) => value = Some(Value::Num(v)),
                Bool(v) => value = Some(Value::Bool(v)),
                Null => value = Some(Value::Null),
                CurlyClose => {
                    insert_in_hash(&mut retval, key, value);
                    break;
                }
                BracketOpen => {
                    value = get_list(iterator);
                }
                BracketClose => {
                    push_in_vec(&mut list, value);
                    println!("List done: {:?}", list);
                    value = Some(Value::List(list.clone()));
                    list.clear();
                }
                Comma => {
                    insert_in_hash(&mut retval, key, value);
                    key = None;
                    value = None;
                }
            }
        } else {
            break;
        }
    }
    retval
}

fn get_list(iterator: &mut impl Iterator<Item = Token>) -> Option<Value> {
    let mut retval = Vec::new();
    loop {
        if let Some(token) = iterator.next() {
            match token {
                CurlyOpen => retval.push(Value::Object(get_json(iterator))),
                Str(v) => retval.push(Value::Str(v)),
                Num(v) => retval.push(Value::Num(v)),
                Bool(v) => retval.push(Value::Bool(v)),
                Null => retval.push(Value::Null),
                Comma => continue,
                BracketClose => break,
                _ => {}
            };
        } else {
            break;
        }
    }
    Some(Value::List(retval))
}

fn push_in_vec(list: &mut Vec<Value>, value: Option<Value>) {
    if let Some(v) = value {
        list.push(v)
    } else {
        panic!("Error: try to push None value in List")
    }
}

fn insert_in_hash(
    hash: &mut HashMap<String, Box<Value>>,
    key: Option<String>,
    value: Option<Value>,
) {
    if let Some(k) = key {
        if let Some(v) = value {
            hash.insert(k, Box::new(v));
        } else {
            panic!("Error: try to insert None value in hash")
        }
    } else {
        panic!("Error: try to insert None key in hash")
    }
}
