use crate::time_tools::{get_freq_estimate, get_rdtsc};

static mut TIMING_POINTS: Vec<(String, u64)> = vec![];

pub fn push_time(name: &str) {
    unsafe {
        TIMING_POINTS.push((name.to_string(), get_rdtsc()));
    }
}

pub fn display_profile() {
    let mut iter;
    let last;
    let freq = get_freq_estimate(100);
    unsafe {
        last = TIMING_POINTS[TIMING_POINTS.len() - 1].clone();
        iter = TIMING_POINTS.iter();
    }
    let total = display_total(&mut iter, last.1, freq);
    display_zones(&mut iter, total, freq);
}

fn display_total<'a>(
    iter: &mut impl Iterator<Item = &'a (String, u64)>,
    last: u64,
    freq: u128,
) -> u64 {
    let start = iter.next().expect("Error: profiling, missing start value");
    let total = last - start.1;
    println!(
        "Total time: {} ms (guessed freq): {}",
        (total as u128) / freq,
        freq
    );

    total
}

fn display_zones<'a>(iter: &mut impl Iterator<Item = &'a (String, u64)>, total: u64, freq: u128) {
    loop {
        let start_point = iter.next();
        if let Some(start_point) = start_point {
            let stop_point = iter.next().expect(&format!(
                "Error: missing end point for start point for: {}",
                start_point.0
            ));
            let elapsed = stop_point.1 - start_point.1;
            println!(
                "{}: {} ms ({:.2})",
                start_point.0,
                (elapsed as u128) / freq,
                100.0 * (elapsed as f64) / (total as f64)
            );
        } else {
            break;
        }
    }
}
