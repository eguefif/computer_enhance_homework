pub mod binary_handler;
pub mod haversine;
pub mod pair;

use crate::binary_handler::get_check_average;
use crate::haversine::compute;
use crate::pair::get_pairs;
use haversine_calculator::json_parse;
use haversine_calculator::Value;
use std::collections::HashMap;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let json = handle_json(&args);
    let pairs = get_pairs(json);
    compute(&pairs);
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
