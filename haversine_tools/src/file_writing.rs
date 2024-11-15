use crate::pair::Pair;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_binary_results(results: &[f64], filename: String) {
    let mut file = File::create(filename).unwrap();
    for res in results.iter() {
        file.write_all(&res.to_ne_bytes()).unwrap();
    }
}

pub fn generate_json_file(filename: String, pairs: &[Pair]) {
    let json = serialize_json(pairs);
    write_file(json, filename);
}

fn serialize_json(pairs: &[Pair]) -> String {
    let mut retval = String::new();
    retval.push_str("{\"pairs\": [");

    let mut iter = pairs.iter().peekable();
    while let Some(pair) = iter.next() {
        let line = add_line(pair);
        retval += &line;
        if iter.peek().is_some() {
            retval += ", ";
        }
    }

    retval.push_str("]}");
    retval
}

fn add_line(pair: &Pair) -> String {
    format!(
        "{{\"x0\":{}, \"y0\":{}, \"x1\": {}, \"y1\": {} }}",
        pair.x0, pair.y0, pair.x1, pair.y1
    )
}

fn write_file(json: String, filename: String) {
    let mut file = File::create(filename).unwrap();
    let bytes = json.into_bytes();
    file.write_all(&bytes).unwrap();
}
