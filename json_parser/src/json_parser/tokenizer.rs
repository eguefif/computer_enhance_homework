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
    Key(String),
    Num(i64),
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
    None
}

fn is_symbol(token: &str) -> bool {
    let symbols = vec!["{", "}", "[", "]", ",", ":", "-"];
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
        "-" => Token::Minus,
        _ => panic!("Impossible: symbol token match error"),
    }
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
    fn it_should_return_a_token_symbol() {
        use super::Token::{BracketClose, BracketOpen, Colon, Comma, CurlyClose, CurlyOpen, Minus};
        let symbols = vec!["{", "}", "[", "]", ",", ":", "-"];
        let expected = [
            CurlyOpen,
            CurlyClose,
            BracketOpen,
            BracketClose,
            Comma,
            Colon,
            Minus,
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
