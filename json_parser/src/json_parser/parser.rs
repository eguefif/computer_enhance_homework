use crate::json_parser::tokenizer::Token;
use crate::json_parser::Pairs;

const OPENINGS: [char; 4] = ['{', '[', '"', ':'];
const CLOSINGS: [char; 4] = ['}', '}', '"', ','];

pub fn parse_tokens(tokens: Vec<Token>) -> Pairs {
    Pairs::new()
}
