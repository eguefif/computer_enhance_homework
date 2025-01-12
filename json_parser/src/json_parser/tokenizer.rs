use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Comma,
    Colon,
    Minus,
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

pub struct Tokens {
    content: Vec<char>,
}

impl Tokens {
    pub fn new(content: String) -> Self {
        Tokens {
            content: sanitize(&content).chars().collect::<Vec<_>>(),
        }
    }
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = String::new();
        loop {
            if let Some(c) = self.content.iter().next() {
                token.push(*c);
            } else {
                break;
            }
            if let retval = get_token(&token)? {
                return Some(retval);
            }
        }
        None
    }
}

fn get_token(token: &str) -> Option<Token> {
    if token.len() == 1 && is_symbol(token) {
        return Some(get_symbol_token(token));
    }
    if is_string(token) {
        let str = String::from(&token[1..token.len() - 1]);
        return Some(Token::Str(str));
    }
    if is_bool(token) {
        match token {
            "true" => return Some(Token::Bool(true)),
            "false" => return Some(Token::Bool(false)),
            _ => panic!("Impossible token value bool"),
        }
    }
    if let Ok(num) = token.parse::<f64>() {
        return Some(Token::Num(num));
    }
    None
}

fn is_bool(token: &str) -> bool {
    if token == "true" || token == "false" {
        return true;
    }
    false
}

fn is_symbol(token: &str) -> bool {
    let symbols = vec!["{", "}", "[", "]", ",", ":"];
    symbols.iter().filter(|x| **x == token).count() == 1
}

fn get_symbol_token(token: &str) -> Token {
    match token {
        "{" => Token::CurlyOpen,
        "}" => Token::CurlyClose,
        "[" => Token::BracketOpen,
        "]" => Token::BracketClose,
        "," => Token::Comma,
        ":" => Token::Colon,
        _ => panic!("Impossible: symbol token match error"),
    }
}

fn is_string(token: &str) -> bool {
    if token.len() > 2 {
        if token.chars().next().unwrap() == '\"' && token.chars().last().unwrap() == '\"' {
            return true;
        }
    }
    false
}

fn sanitize(content: &str) -> String {
    let mut retval = String::with_capacity(content.len());
    content.chars().for_each(|x| {
        if x != '\n' && x != '\t' && x != ' ' && x != '\r' {
            retval.push(x);
        }
    });
    retval
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_a_num_token() {
        let result = get_token("-123.122").unwrap();
        assert_eq!(result, Token::Num(-123.122f64));
    }

    #[test]
    fn it_should_return_a_bool_token_true() {
        let result = get_token("true").unwrap();
        assert_eq!(result, Token::Bool(true));
    }
    #[test]
    fn it_should_return_a_bool_token_false() {
        let result = get_token("false").unwrap();
        assert_eq!(result, Token::Bool(false));
    }

    #[test]
    fn it_should_return_a_str_token() {
        let result = get_token("\"hello world str\"").unwrap();
        assert_eq!(result, Token::Str("hello world str".to_string()));
    }

    #[test]
    fn it_should_return_a_token_symbol() {
        use super::Token::{BracketClose, BracketOpen, Colon, Comma, CurlyClose, CurlyOpen};
        let symbols = vec!["{", "}", "[", "]", ",", ":"];
        let expected = [
            CurlyOpen,
            CurlyClose,
            BracketOpen,
            BracketClose,
            Comma,
            Colon,
        ];
        for (i, s) in symbols.iter().enumerate() {
            assert_eq!(expected[i], get_token(s).unwrap())
        }
    }

    #[test]
    fn it_should_sanitize() {
        let content = String::from(
            "{\"pairs\":\n    [ {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123 }, {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123  ,   \n \"y1\": 32.123}\n ]}",
        );

        let result = sanitize(&content);
        let expected = String::from(
            "{\"pairs\":[{\"x0\":312.31,\"x1\":32.123,\"y0\":-32.123,\"y1\":32.123},{\"x0\":312.31,\"x1\":32.123,\"y0\":-32.123,\"y1\":32.123}]}",
        );
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn it_should_tokenize() {
        let content = String::from(
            "{\"pairs\": [ {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123 }, {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123} ]}",
        );
        let expected: Vec<&str> = vec![
            "pairs", ":", "[", "{", "x0", ":", "312.31", ",", "x1", ":", "32.123", ",", "y0", ":",
            "-32.123", ",", "y1", ":", "32.123", "}", "{", "x0", ":", "312.31", ",", "x1", ":",
            "32.123", ",", "y0", ":", "-32.123", ",", "y1", ":", "32.123", "}", "]",
        ];
    }
}
