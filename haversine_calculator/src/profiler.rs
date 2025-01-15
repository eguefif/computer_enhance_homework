#![allow(static_mut_refs)]
use crate::time_tools::{get_freq_estimate, get_rdtsc};

struct Zone {
    elapsed: u64,
    label: String,
    parent: String,
}

static mut CURRENT_PARENT: String = String::new();

static mut TIMING_POINTS: Vec<Zone> = vec![];

pub fn begin_profiling() {
    let zone = Zone {
        elapsed: get_rdtsc(),
        label: "start".to_string(),
        parent: "".to_string(),
    };
    unsafe {
        TIMING_POINTS.push(zone);
        CURRENT_PARENT.push_str("main");
    }
}

pub fn push_time(name: String, time: u64, parent: String) {
    let zone = Zone {
        elapsed: time,
        label: name,
        parent,
    };
    unsafe {
        TIMING_POINTS.push(zone);
    }
}

pub fn get_profiling_parent() -> String {
    unsafe { CURRENT_PARENT.clone() }
}

pub fn set_profiling_parent(parent: String) {
    unsafe { CURRENT_PARENT = parent };
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

fn display_total<'a>(iter: &mut impl Iterator<Item = &'a Zone>, last: u64, freq: u128) -> u64 {
    let start = iter.next().expect("Error: profiling, missing start value");
    let total = last - start.elapsed;
    println!(
        "\nTotal time: {} ms (guessed freq): {}",
        (total as u128) / freq,
        freq
    );

    total
}

fn display_zones<'a>(iter: &mut impl Iterator<Item = &'a Zone>, total: u64, freq: u128) {
    loop {
        let zone = iter.next();
        if let Some(zone) = zone {
            println!(
                "{}: {} ms ({:.2}%) parent: {}",
                zone.label,
                (zone.elapsed as u128) / freq,
                100.0 * (zone.elapsed as f64) / (total as f64),
                zone.parent,
            );
        } else {
            break;
        }
    }
}
