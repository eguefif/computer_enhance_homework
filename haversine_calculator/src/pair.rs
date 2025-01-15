use crate::parser::Value;
use crate::profiler::{get_profiling_parent, push_time, set_profiling_parent};
use crate::time_tools::get_rdtsc;
use profile::zone;
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

#[zone]
pub fn get_pairs(json: HashMap<String, Box<Value>>) -> Vec<Pair> {
    let mut retval = Vec::new();
    let list = json
        .get("pairs")
        .expect("Error: need identifier pairs in json")
        .clone();
    if let Value::List(pairs) = *list {
        for pair in pairs {
            if let Some(p) = get_pair(&pair) {
                retval.push(p);
            } else {
                panic!("Error: error pair invalid {:?}", pair);
            }
        }
    }
    retval
}

fn get_pair(pair: &Value) -> Option<Pair> {
    if let Value::Object(p) = pair {
        let x0_val = p
            .get("x0")
            .expect("Error: pair invalid, missing x0")
            .clone();
        let x1_val = p
            .get("x1")
            .expect("Error: pair invalid, missing x1")
            .clone();
        let y0_val = p
            .get("y0")
            .expect("Error: pair invalid, missing y0")
            .clone();
        let y1_val = p
            .get("y1")
            .expect("Error: pair invalid, missing y1")
            .clone();
        let x0;
        let x1;
        let y0;
        let y1;

        if let Value::Num(val) = *x0_val {
            x0 = val;
        } else {
            panic!("Error: x0 is not a number");
        }
        if let Value::Num(val) = *x1_val {
            x1 = val;
        } else {
            panic!("Error: x1 is not a number");
        }
        if let Value::Num(val) = *y0_val {
            y0 = val;
        } else {
            panic!("Error: y0 is not a number");
        }
        if let Value::Num(val) = *y1_val {
            y1 = val;
        } else {
            panic!("Error: y1 is not a number");
        }
        return Some(Pair::new(x0, x1, y0, y1));
    }
    None
}
