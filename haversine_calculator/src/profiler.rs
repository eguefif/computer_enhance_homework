#![allow(static_mut_refs)]
use crate::time_tools::{get_freq_estimate, get_rdtsc};

#[derive(Debug)]
struct Zone {
    elapsed: u64,
    label: String,
    child_time: u64,
}

static mut CURRENT_PARENT: String = String::new();

static mut ZONES: Vec<Zone> = vec![];

pub fn begin_profiling() {
    let zone = Zone {
        elapsed: get_rdtsc(),
        label: "start".to_string(),
        child_time: 0,
    };
    unsafe {
        ZONES.push(zone);
    }
}

pub fn update_parent(name: String, time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.child_time += time;
            }
        }
    }
}

pub fn push_time(name: String, time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.elapsed += time;
                return;
            }
        }
    }
    let zone = Zone {
        elapsed: time,
        label: name,
        child_time: 0,
    };
    unsafe {
        ZONES.push(zone);
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
        iter = ZONES.iter();
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
    let mut actual_count = 0;
    for zone in iter {
        let actual_total = zone.elapsed - zone.child_time;
        actual_count += actual_total;
        println!(
            "{:>15}:{:>7}ms ({:3.2}%, {:3.2}%) {:?}",
            zone.label,
            (zone.elapsed as u128) / freq,
            100.0 * (actual_total as f64) / (total as f64),
            100.0 * (zone.elapsed as f64) / (total as f64),
            zone
        );
    }
    println!(
        "Actual total: {:3.2} %",
        (actual_count as f64) / (total as f64) * 100.0
    );
}
