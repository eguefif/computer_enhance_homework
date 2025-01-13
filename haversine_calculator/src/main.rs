pub mod binary_handler;
pub mod haversine;
pub mod pair;
pub mod time_tools;

use crate::binary_handler::get_check_average;
use crate::haversine::compute;
use crate::pair::get_pairs;
use crate::time_tools::get_freq_estimate;
use haversine_calculator::json_parse;
use std::time::Instant;

use std::env;
use std::fs;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let filename = &args[1];
    let setup = start.elapsed().as_millis();

    let content = fs::read_to_string(filename).expect("Error while reading json file");
    let file_loading = start.elapsed().as_millis();

    let json = json_parse(content);
    let pairs = get_pairs(json);
    let parsing = start.elapsed().as_millis();

    let json_average = compute(&pairs);
    let compute = start.elapsed().as_millis();
    display_profile(setup, file_loading, parsing, compute as f64);
    handle_binary(&args, json_average);
}

fn display_profile(setup: u128, file_loading: u128, parsing: u128, total: f64) {
    println!(
        "Total time: {total} (guessed freq: {}",
        get_freq_estimate(100)
    );
    println!(
        "setup: {}ms ({:.2}%)",
        setup,
        (setup as f64) / total * 100.0
    );
    let loading = file_loading - setup;
    println!("loading: {}ms ({:.2}%)", loading, (loading as f64) / total);
    let parse = parsing - file_loading;
    println!(
        "parsing: {}ms ({:.2}%)",
        parse,
        (parse as f64) / total * 100.0
    );
    let comp = (total as u128) - parsing;
    println!(
        "computing: {}ms ({:.2}%)",
        comp,
        (comp as f64) / total * 100.0
    );
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
