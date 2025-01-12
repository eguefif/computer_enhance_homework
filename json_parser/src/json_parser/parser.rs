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

enum CollectionType {
    Hash,
    List,
}

fn get_json(iterator: &mut impl Iterator<Item = Token>) -> HashMap<String, Box<Value>> {
    let mut retval = HashMap::new();
    let mut key: Option<String> = None;
    let mut value: Option<Value> = None;
    let mut list: Option<Vec<Value>> = None;
    let mut collection = CollectionType::Hash;
    loop {
        if let Some(token) = iterator.next() {
            match token {
                CurlyOpen => value = Some(Value::Object(get_json(iterator))),
                CurlyClose => break,
                BracketOpen => {
                    collection = CollectionType::List;
                }
                BracketClose => {
                    collection = CollectionType::Hash;
                    if let Some(v) = value {
                        if let Some(l) = list {
                            l.push(v.clone());
                            if let Some(k) = key {
                                retval.insert(k.clone(), Box::new(Value::List(l.clone())));
                                list = None;
                            }
                        } else {
                            panic!("Error: push list was empty but it tried to push");
                        }
                    }
                }
                Comma => match collection {
                    CollectionType::Hash => {
                        if let Some(k) = key {
                            if let Some(v) = value {
                                retval.insert(k.clone(), Box::new(v));
                                key = None;
                                value = None;
                            } else {
                                panic!("Error: try to push in hashmap but there is no value");
                            }
                        } else {
                            panic!("Error: try to push in hashmap but there is no key");
                        }
                    }
                    CollectionType::List => {
                        if let Some(mut l) = list {
                            if let Some(v) = value {
                                l.push(v.clone());
                            }
                        } else {
                            panic!("Error: try to push on list but list is None");
                        }
                    }
                },
                Key(v) => key = Some(v),
                Str(v) => value = Some(Value::Str(v)),
                Num(v) => value = Some(Value::Num(v)),
                Bool(v) => value = Some(Value::Bool(v)),
                Null => value = Some(Value::Null),
            }
        } else {
            break;
        }
    }
    retval
}
