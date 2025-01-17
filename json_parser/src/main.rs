pub mod binary_handler;

use crate::binary_handler::get_check_average;
use json_parser::json_parse;
use json_parser::Value;
use std::collections::HashMap;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 && args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let json = handle_json(&args);
    println!("{:?}", json);
    handle_binary(&args);
}

fn handle_json(args: &[String]) -> HashMap<String, Box<Value>> {
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
