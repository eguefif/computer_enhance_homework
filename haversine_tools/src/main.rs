pub mod file_writing;
pub mod haversine;
pub mod pair;

use crate::haversine::compute;
use std::env;

use pair::get_pairs;

use crate::file_writing::generate_binary_results;
use crate::file_writing::generate_json_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: cargo run -- X filename");
        return;
    }
    let quantity: u128 = String::from(&args[1])
        .parse()
        .expect("usage: cargo run -- [quantity] filename");
    let pairs = get_pairs(quantity);

    let results = compute(&pairs);

    if args.len() == 3 {
        let filename = String::from(&args[2]);
        let binary_filename = format!("{}-f64", filename);
        let json_filename = format!("{}.json", filename);
        generate_json_file(json_filename, &pairs);
        generate_binary_results(&results, binary_filename);
    }
}
