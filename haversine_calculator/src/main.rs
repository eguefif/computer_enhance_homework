pub mod binary_handler;
pub mod haversine;
pub mod pair;
pub mod time_tools;

use crate::binary_handler::get_check_average;
use crate::haversine::compute;
use crate::pair::get_pairs;
use crate::time_tools::{get_freq_estimate, get_rdtsc};
use haversine_calculator::json_parse;

use std::env;
use std::fs;

fn main() {
    let start = get_rdtsc();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- file.json [binary.f64]");
        return;
    }
    let filename = &args[1];
    let setup = get_rdtsc();

    let content = fs::read_to_string(filename).expect("Error while reading json file");
    let file_loading = get_rdtsc();

    let json = json_parse(content);
    let pairs = get_pairs(json);
    let parsing = get_rdtsc();

    let json_average = compute(&pairs);
    let compute = get_rdtsc();
    display_profile(start, setup, file_loading, parsing, compute);
    handle_binary(&args, json_average);
}

fn display_profile(start: u64, setup: u64, file_loading: u64, parsing: u64, compute: u64) {
    let total = compute - start;
    println!(
        "Total time: {total} (guessed freq): {}",
        get_freq_estimate(100)
    );
    let setup_elapsed = setup - start;
    println!(
        "setup: {}ms ({:.2}%)",
        setup_elapsed,
        (setup_elapsed as f64) / (total as f64) * 100.0
    );
    let loading_elapsed = file_loading - setup;
    println!(
        "loading: {}ms ({:.2}%)",
        loading_elapsed,
        100.0 * (loading_elapsed as f64) / (total as f64)
    );
    let parse = parsing - file_loading;
    println!(
        "parsing: {}ms ({:.2}%)",
        parse,
        100.0 * (parse as f64) / (total as f64)
    );
    let comp = compute - parsing;
    println!(
        "computing: {}ms ({:.2}%)",
        comp,
        100.0 * (comp as f64) / (total as f64)
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
