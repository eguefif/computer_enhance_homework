pub mod binary_handler;
pub mod haversine;
pub mod pair;
pub mod parser;
pub mod profiler;
pub mod time_tools;

use crate::binary_handler::get_check_average;
use crate::haversine::compute;
use crate::pair::get_pairs;
use crate::parser::json_parse;
use profile::profile;

use std::env;
use std::fs;

#[profile]
fn main() {
    if let Some(content) = get_content() {
        let json = json_parse(content);
        let pairs = get_pairs(json);

        let json_average = compute(&pairs);
        handle_binary(json_average);
    }
}

fn get_content() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return None;
    }
    let filename = &args[1];

    Some(fs::read_to_string(filename).expect("Error while reading json file"))
}

fn handle_binary(json_average: f64) {
    let binary;
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        binary = &args[2];
        let check_average: f64 = get_check_average(binary);
        println!("Validation");
        println!("Reference sum: {}", check_average);
        println!("Diff: {}", check_average - json_average);
    }
}
