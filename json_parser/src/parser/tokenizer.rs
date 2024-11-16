pub fn tokenize(content: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];

    let mut buffer = String::new();
    let mut value_flag = false;
    let mut key_flag = false;
    let mut array_flag = false;

    for ch in content.chars() {
        if ch != ' ' || ch != '\n' {
            buffer.push(ch);
        } else {
            continue;
        }
        match ch {
            '{' => {
                tokens.push(String::from("{"));
            }
            '}' => {
                tokens.push(String::from("}"));
            }
            '[' => {
                array_flag = true;
                tokens.push(String::from("["));
            }
            ']' => {
                if !array_flag || value_flag || key_flag {
                    panic!("Error tokenizing: list start in the middle of a list or value")
                }
                let token = &buffer[0..buffer.len() - 1];
                tokens.push(String::from(token.trim()));
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
                if !value_flag || key_flag {
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
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_tokenize() {
        let content = String::from(
            "{\"pairs\": [ {\"x0\": 312.31, \"x1\": 32.123, \"y0\": -32.123, \"y1\": 32.123,]",
        );
        let res = tokenize(content);
        println!("res{:?}", res);
    }
}
