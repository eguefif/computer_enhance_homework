#![allow(static_mut_refs)]
use crate::time_tools::{get_freq_estimate, get_rdtsc};

#[derive(Debug)]
struct Zone {
    elapsed: u64,
    root_elapsed: u64,
    label: String,
    child_elapsed: u64,
    hit_count: u128,
}

static mut CURRENT_PARENT: String = String::new();

static mut ZONES: Vec<Zone> = vec![];

pub fn begin_profiling() {
    let zone = Zone {
        elapsed: get_rdtsc(),
        root_elapsed: 0,
        label: "start".to_string(),
        child_elapsed: 0,
        hit_count: 0,
    };
    unsafe {
        ZONES.push(zone);
    }
}

pub fn create_zone(name: String) {
    if is_zone(&name) {
        return;
    }
    let zone = Zone {
        elapsed: 0,
        label: name,
        child_elapsed: 0,
        root_elapsed: 0,
        hit_count: 0,
    };
    unsafe {
        ZONES.push(zone);
    }
}

fn is_zone(name: &str) -> bool {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                return true;
            }
        }
    }
    false
}

pub fn get_root_elapsed(name: &str) -> u64 {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                return zone.root_elapsed;
            }
        }
    }
    0
}

pub fn update_parent(name: String, time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.child_elapsed += time;
            }
        }
    }
}

pub fn push_time(name: String, time: u64, root_time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.elapsed += time;
                zone.root_elapsed = time + root_time;
                zone.hit_count += 1;
                return;
            }
        }
    }
    panic!("Error: zone should have already been created");
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
    let mut total_count = 0;
    for zone in iter {
        let actual_total = zone.elapsed - zone.child_elapsed;
        actual_count += actual_total;
        total_count += zone.root_elapsed;
        print!(
            "{:>23}:{:>7}ms ({:3.2}%",
            format!("{}({})", zone.label, zone.hit_count),
            (zone.elapsed as u128) / freq,
            100.0 * (actual_total as f64) / (total as f64),
        );
        if actual_total != zone.root_elapsed {
            print!(
                ", {:3.2}% with children",
                100.0 * (zone.root_elapsed as f64) / (total as f64)
            );
        }
        println!(")");
    }
    println!(
        "Actual total: {:3.2} %",
        (actual_count as f64) / (total as f64) * 100.0
    );
    println!(
        "Total: {:3.2} %",
        (total_count as f64) / (total as f64) * 100.0
    );
}
