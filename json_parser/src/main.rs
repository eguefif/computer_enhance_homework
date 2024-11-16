pub mod binary_handler;
pub mod json_parser;

use json_parser::Pairs;

use crate::binary_handler::get_check_average;
use crate::json_parser::json_parse;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 && args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let pairs = handle_json(&args);
    handle_binary(&args);
}

fn handle_json(args: &[String]) -> Pairs {
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Error while reading json file");
    json_parse(content)
}

fn handle_binary(args: &[String]) {
    let binary;
    if args.len() == 3 {
        binary = &args[2];
        let check_average: f64 = get_check_average(binary);
        println!("Validation");
        println!("Reference sum: {}", check_average);
        println!("Diff: {}", check_average);
    }
}
