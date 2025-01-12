use haversine_calculator::Value;
use std::collections::HashMap;
#[derive(PartialEq)]
pub struct Pair {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Pair {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64) -> Pair {
        Pair { x0, x1, y0, y1 }
    }
}

pub fn get_pairs(json: HashMap<String, Box<Value>>) -> Vec<Pair> {
    let retval = Vec::new();
    retval
}
