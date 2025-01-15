#[allow(static_mut_refs)]
use crate::time_tools::{get_freq_estimate, get_rdtsc};

static mut TIMING_POINTS: Vec<(String, u64)> = vec![];

pub fn begin_profiling() {
    unsafe {
        TIMING_POINTS.push(("start".to_string(), get_rdtsc()));
    }
}

pub fn push_time(name: &str, time: u64) {
    unsafe {
        TIMING_POINTS.push((name.to_string(), time));
    }
}

pub fn display_profile() {
    let last = get_rdtsc();
    let mut iter;
    let freq = get_freq_estimate(100);
    unsafe {
        iter = TIMING_POINTS.iter();
    }
    let total = display_total(&mut iter, last, freq);
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
        "\nTotal time: {} ms (guessed freq): {}",
        (total as u128) / freq,
        freq
    );

    total
}

fn display_zones<'a>(iter: &mut impl Iterator<Item = &'a (String, u64)>, total: u64, freq: u128) {
    loop {
        let zone = iter.next();
        if let Some(zone) = zone {
            println!(
                "{}: {} ms ({:.2}%)",
                zone.0,
                (zone.1 as u128) / freq,
                100.0 * (zone.1 as f64) / (total as f64)
            );
        } else {
            break;
        }
    }
}
