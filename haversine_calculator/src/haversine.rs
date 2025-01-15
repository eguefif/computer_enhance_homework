const EARTH_RADIUS: f64 = 6372.8;
use crate::pair::Pair;
use crate::profiler::{get_profiling_parent, push_time, set_profiling_parent};
use crate::time_tools::get_rdtsc;
use profile::zone;

#[zone]
pub fn compute(pairs: &[Pair]) -> f64 {
    let mut results: Vec<f64> = vec![];
    for pair in pairs.iter() {
        let res = compute_haversine(pair.x0, pair.x1, pair.y0, pair.y1);
        results.push(res);
    }
    println!("Method: cluster");
    println!("Pair count: {}", results.len());
    let av = average(&results);
    println!("Pair count: {}", av);
    av
}

#[allow(clippy::excessive_precision)]
fn radians_from_degrees(value: f64) -> f64 {
    0.017_453_292_519_943_295_77 * value
}

fn square(value: f64) -> f64 {
    value * value
}

fn compute_haversine(x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let mut lat1 = y0;
    let mut lat2 = y1;
    let lon1 = x0;
    let lon2 = x1;

    let d_lat = radians_from_degrees(lat2 - lat1);
    let d_lon = radians_from_degrees(lon2 - lon1);

    lat1 = radians_from_degrees(lat1);
    lat2 = radians_from_degrees(lat2);

    let a = square((d_lat / 2.0).sin()) + lat1.cos() * lat2.cos() * square((d_lon / 2.0).sin());
    let c = 2.0 * (a.sqrt()).asin();
    EARTH_RADIUS * c
}

fn average(results: &[f64]) -> f64 {
    let mut acc: f64 = 0.0;
    for res in results.iter() {
        acc += res;
    }
    acc / results.len() as f64
}
