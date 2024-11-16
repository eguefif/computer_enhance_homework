use crate::json_parser::Pairs;

const OPENINGS: [char; 4] = ['{', '[', '"', ':'];
const CLOSINGS: [char; 4] = ['}', '}', '"', ','];

pub fn parse_tokens(tokens: Vec<String>) -> Pairs {
    Pairs::new()
}
