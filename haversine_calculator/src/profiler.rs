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
    println!("{:^15}: {:^8} ({}, {})", "label", "time", "actual", "total");
    println!("-------------------------------------------");
    loop {
        let zone = iter.next();
        if let Some(zone) = zone {
            let child_time = get_child_time(&zone.label);
            let actual_total = zone.elapsed - child_time;
            println!(
                "{:>15}:{:>7}ms ({:3.2}%, {:3.2}%)",
                zone.label,
                (zone.elapsed as u128) / freq,
                100.0 * (actual_total as f64) / (total as f64),
                100.0 * (zone.elapsed as f64) / (total as f64),
            );
        } else {
            break;
        }
    }
}

fn get_child_time(label: &str) -> u64 {
    let mut counter = 0;
    unsafe {
        for zone in TIMING_POINTS.iter() {
            if zone.parent == label {
                counter += zone.elapsed;
            }
        }
    }
    counter
}
