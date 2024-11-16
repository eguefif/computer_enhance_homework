pub mod binary_handler;
pub mod parser;

use parser::json_parse;

use crate::binary_handler::get_check_average;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 && args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let filename = &args[1];

    let _ = fs::read_to_string(filename).expect("Error while reading json file");

    let binary;
    if args.len() == 3 {
        binary = &args[2];
        let check_average: f64 = get_check_average(binary);
        println!("Validation");
        println!("Reference sum: {}", check_average);
        println!("Diff: {}", check_average);
    }
}
