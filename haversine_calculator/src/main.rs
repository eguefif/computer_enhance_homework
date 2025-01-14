pub mod binary_handler;
pub mod haversine;
pub mod pair;
pub mod profiler;
pub mod time_tools;

use crate::binary_handler::get_check_average;
use crate::haversine::compute;
use crate::pair::get_pairs;
use crate::profiler::{display_profile, push_time};
use haversine_calculator::json_parse;

use std::env;
use std::fs;

fn main() {
    push_time("start");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("Error while reading json file");

    push_time("parsing");

    let json = json_parse(content);
    let pairs = get_pairs(json);

    push_time("parsing stop");

    push_time("compute");
    let json_average = compute(&pairs);
    push_time("compute stop");
    handle_binary(&args, json_average);
    display_profile()
}

fn handle_binary(args: &[String], json_average: f64) {
    let binary;
    if args.len() == 3 {
        binary = &args[2];
        let check_average: f64 = get_check_average(binary);
        println!("Validation");
        println!("Reference sum: {}", check_average);
        println!("Diff: {}", check_average - json_average);
    }
}
