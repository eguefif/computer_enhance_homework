use std::fs::File;
use std::io::prelude::*;

pub fn get_check_average(filename: &str) -> f64 {
    let mut file = File::open(filename).expect("Error while reading binary file");
    let mut buffer: Vec<u8> = vec![];

    let res = file.read_to_end(&mut buffer);
    match res {
        Ok(_) => get_average(buffer),
        Err(_) => panic!("Error while reading file"),
    }
}

fn get_average(buffer: Vec<u8>) -> f64 {
    let mut results: Vec<f64> = vec![];

    for n in 0..buffer.len() {
        let mut arr: [u8; 8] = [0; 8];
        if n % 8 == 0 {
            arr.copy_from_slice(&buffer[n..n + 8]);
            let value = f64::from_ne_bytes(arr);
            results.push(value);
        }
    }
    compute_average(results)
}

fn compute_average(results: Vec<f64>) -> f64 {
    let mut acc: f64 = 0.0;
    for r in results.iter() {
        acc += r;
    }

    acc / results.len() as f64
}
