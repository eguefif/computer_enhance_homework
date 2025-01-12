// TODO: add Key to enum. A key always end by : and remove :
#[derive(Debug, PartialEq)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Comma,
    Key(String),
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let sanitized_content = sanitize(content);
    get_tokens(&sanitized_content)
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

fn get_tokens(content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = content.chars().peekable();
    let mut token = String::new();

    loop {
        if let Some(c) = chars.next() {
            token.push(c);
        } else {
            break;
        }
        if let Some(retval) = get_token(&token, chars.peek()) {
            tokens.push(retval);
            token.clear();
        }
    }
    tokens
}

fn get_token(token: &str, next: Option<&char>) -> Option<Token> {
    if is_symbol(token) {
        return Some(get_symbol_token(token));
    }

    if is_string(token) {
        let str = String::from(&token[1..token.len() - 1]);
        if let Some(next_char) = next {
            if *next_char == ':' {
                return Some(Token::Key(str));
            }
        }
        return Some(Token::Str(str));
    }

    if is_bool(token) {
        match token {
            "true" => return Some(Token::Bool(true)),
            "false" => return Some(Token::Bool(false)),
            _ => panic!("Impossible: bool token value error"),
        }
    }

    if is_number(token, next) {
        println!("{:?}", next);
        if let Ok(num) = token.parse::<f64>() {
            return Some(Token::Num(num));
        }
    }
    if token == "null" {
        return Some(Token::Null);
    }
    None
}

fn is_symbol(token: &str) -> bool {
    let symbols = vec!["{", "}", "[", "]", ","];
    symbols.iter().filter(|x| **x == token).count() == 1
}

fn get_symbol_token(token: &str) -> Token {
    match token {
        "{" => Token::CurlyOpen,
        "}" => Token::CurlyClose,
        "[" => Token::BracketOpen,
        "]" => Token::BracketClose,
        "," => Token::Comma,
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

fn is_bool(token: &str) -> bool {
    if token == "true" || token == "false" {
        return true;
    }
    false
}

fn is_number(token: &str, next: Option<&char>) -> bool {
    if token.len() > 2 {
        let mut chars = token.chars();
        let first = chars.next().unwrap();
        if let Some(last) = next {
            if (first.is_digit(10) || first == '-' || first == '+')
                && (*last == ',' || *last == ']' || *last == '}')
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_a_none() {
        let result = get_token("fsdf", None);
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_return_a_num_token_minus() {
        let result = get_token("-123.122", Some(&'}')).unwrap();
        assert_eq!(result, Token::Num(-123.122f64));
    }

    #[test]
    fn it_should_return_a_num_token_plus() {
        let result = get_token("+123.122", Some(&']')).unwrap();
        assert_eq!(result, Token::Num(123.122f64));
    }

    #[test]
    fn it_should_return_a_num_token_int() {
        let result = get_token("123", Some(&',')).unwrap();
        assert_eq!(result, Token::Num(123.0f64));
    }

    #[test]
    fn it_should_return_a_bool_token_true() {
        let result = get_token("true", None).unwrap();
        assert_eq!(result, Token::Bool(true));
    }
    #[test]
    fn it_should_return_a_bool_token_false() {
        let result = get_token("false", None).unwrap();
        assert_eq!(result, Token::Bool(false));
    }

    #[test]
    fn it_should_return_a_str_token() {
        let result = get_token("\"hello world str\"", None).unwrap();
        assert_eq!(result, Token::Str("hello world str".to_string()));
    }

    #[test]
    fn it_should_return_a_token_symbol() {
        use super::Token::{BracketClose, BracketOpen, Comma, CurlyClose, CurlyOpen};
        let symbols = vec!["{", "}", "[", "]", ","];
        let expected = [CurlyOpen, CurlyClose, BracketOpen, BracketClose, Comma];
        for (i, s) in symbols.iter().enumerate() {
            assert_eq!(expected[i], get_token(s, None).unwrap())
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
    fn it_should_tokenize() {
        use super::Token::{
            Bool, BracketClose, BracketOpen, Comma, CurlyClose, CurlyOpen, Key, Null, Num, Str,
        };
        let content = String::from(
            "{\"pairs\": [ {\"x0\": 3.1, \"x1\": 3.3  }, { \"Hello\": \"world\", \"bool\": true, \"null\": null } ] }"
        );
        let expected: Vec<Token> = vec![
            CurlyOpen,
            Key("pairs".to_string()),
            BracketOpen,
            CurlyOpen,
            Key("x0".to_string()),
            Num(3.1f64),
            Comma,
            Key("x1".to_string()),
            Num(3.3f64),
            CurlyClose,
            Comma,
            CurlyOpen,
            Key("Hello".to_string()),
            Str("world".to_string()),
            Comma,
            Key("bool".to_string()),
            Bool(true),
            Comma,
            Key("null".to_string()),
            Null,
            CurlyClose,
            BracketClose,
            CurlyClose,
        ];

        let result = tokenize(&content);
        for (i, item) in result.iter().enumerate() {
            assert_eq!(*item, expected[i]);
        }
    }
}
