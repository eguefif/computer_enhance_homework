pub fn tokenize(content: String) -> Vec<String> {
    let retval: Vec<String> = vec![];
    retval
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
