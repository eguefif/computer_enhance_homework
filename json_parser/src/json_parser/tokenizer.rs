pub fn tokenize(content: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];

    let mut buffer = String::new();
    let mut value_flag = false;
    let mut key_flag = false;
    let mut array_flag = false;
    let mut bracket_flag = false;

    for ch in content.chars() {
        if ch != ' ' || ch != '\n' {
            buffer.push(ch);
        } else {
            continue;
        }
        match ch {
            '{' => {
                value_flag = !value_flag;
                bracket_flag = true;
                tokens.push(String::from("{"));
            }
            '}' => {
                bracket_flag = false;
                if value_flag {
                    let token = &buffer[0..buffer.len() - 1];
                    tokens.push(String::from(token.trim()));
                    value_flag = !value_flag;
                }
                tokens.push(String::from("}"));
            }
            '[' => {
                value_flag = !value_flag;
                array_flag = true;
                tokens.push(String::from("["));
            }
            ']' => {
                if !array_flag {
                    panic!("Error tokenizing: list start in the middle of a list or value")
                }
                if value_flag {
                    let token = &buffer[0..buffer.len() - 1];
                    tokens.push(String::from(token.trim()));
                    value_flag = !value_flag;
                }
                tokens.push(String::from("]"));
                array_flag = false;
            }
            '"' => {
                if key_flag {
                    let token = &buffer[0..buffer.len() - 1];
                    tokens.push(String::from(token.trim()));
                }
                key_flag = !key_flag;
            }
            ':' => {
                value_flag = true;
                tokens.push(String::from(":"));
            }
            ',' => {
                if (!array_flag || !bracket_flag) && !value_flag {
                    continue;
                }
                if !value_flag {
                    panic!("Error tokenizing: value does not end with a coma")
                }
                let token = &buffer[0..buffer.len() - 1];
                tokens.push(String::from(token.trim()));
                tokens.push(String::from(","));

                value_flag = false;
            }
            _ => continue,
        }
        buffer.clear();
    }
    tokens.pop();
    tokens.remove(0);
    println!("tokens");
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_tokenize() {
        let content = String::from(
            "{\"pairs\": [ {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123 }, {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123} ]}",
        );
        let res = tokenize(content);
        let expected: Vec<&str> = vec![
            "pairs", ":", "[", "{", "x0", ":", "312.31", ",", "x1", ":", "32.123", ",", "y0", ":",
            "-32.123", ",", "y1", ":", "32.123", "}", "{", "x0", ":", "312.31", ",", "x1", ":",
            "32.123", ",", "y0", ":", "-32.123", ",", "y1", ":", "32.123", "}", "]",
        ];
        for (idx, s) in expected.iter().enumerate() {
            assert_eq!(*s, res[idx])
        }
    }
}
