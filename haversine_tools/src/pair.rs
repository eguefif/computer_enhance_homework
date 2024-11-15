use rand::prelude::*;

pub fn get_pairs(upper_bound: u128) -> Vec<Pair> {
    let mut retval: Vec<Pair> = vec![];
    for _ in 0..upper_bound {
        retval.push(Pair::new());
    }
    retval
}

#[derive(PartialEq)]
pub struct Pair {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Pair {
    pub fn new() -> Pair {
        let p1 = get_one_pair();
        let p2 = get_one_pair();
        Pair {
            x0: p1.0,
            x1: p2.0,
            y0: p1.1,
            y1: p2.1,
        }
    }
}

impl Default for Pair {
    fn default() -> Self {
        Self::new()
    }
}

fn get_one_pair() -> (f64, f64) {
    let mut rng = thread_rng();

    let v = rng.gen_range(0..4);
    let x_range = get_x_range(v);
    let y_range = get_y_range(v);

    let x: f64 = rng.gen_range(x_range.0..x_range.1);
    let y: f64 = rng.gen_range(y_range.0..y_range.1);
    (x, y)
}
const X_RANGES1: [f64; 4] = [-86.0, -14.321, 40.0, 85.0];
const X_RANGES2: [f64; 4] = [-4.0, 1.321, 50.0, 88.0];

const Y_RANGES1: [f64; 4] = [-172.0, 34.0, 110.0, 80.0];
const Y_RANGES2: [f64; 4] = [-44.0, 50.0, 123.0, 88.0];

fn get_x_range(v: usize) -> (f64, f64) {
    (X_RANGES1[v], X_RANGES2[v])
}
fn get_y_range(v: usize) -> (f64, f64) {
    (Y_RANGES1[v], Y_RANGES2[v])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_rand_pais() {
        let (x, y) = get_one_pair();
        assert!(x > -91.0);
        assert!(x < 91.0);
        assert!(y > -181.0);
        assert!(y < 181.0);
    }
}
