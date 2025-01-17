#![allow(static_mut_refs)]
use crate::time_tools::{get_freq_estimate, get_rdtsc};

#[derive(Debug)]
struct Zone {
    child_exclusive_elapsed: u64,
    child_inclusive_elapsed: u64,
    label: String,
    hit_count: u128,
}

static mut CURRENT_PARENT: String = String::new();

static mut ZONES: Vec<Zone> = vec![];

pub fn begin_profiling() {
    let zone = Zone {
        child_exclusive_elapsed: get_rdtsc(),
        child_inclusive_elapsed: 0,
        label: "start".to_string(),
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
        child_exclusive_elapsed: 0,
        label: name,
        child_inclusive_elapsed: 0,
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

pub fn get_inclusive_elapsed(name: &str) -> u64 {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                return zone.child_inclusive_elapsed;
            }
        }
    }
    0
}

pub fn update_parent(name: String, time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.child_exclusive_elapsed -= time;
            }
        }
    }
}

pub fn push_time(name: String, time: u64, root_time: u64) {
    unsafe {
        for zone in ZONES.iter_mut() {
            if zone.label == name {
                zone.child_exclusive_elapsed += time;
                zone.child_inclusive_elapsed = time + root_time;
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
    let total = last - start.child_exclusive_elapsed;
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
    for zone in iter {
        print!(
            "{:>23}:{:>7}ms ({:3.2}%",
            format!("{}({})", zone.label, zone.hit_count),
            (zone.child_exclusive_elapsed as u128) / freq,
            100.0 * (zone.child_exclusive_elapsed as f64) / (total as f64),
        );
        if zone.child_exclusive_elapsed != zone.child_inclusive_elapsed {
            print!(
                ", {:3.2}% with children",
                100.0 * (zone.child_inclusive_elapsed as f64) / (total as f64)
            );
        }
        println!(")");
    }
}
