use core::arch::x86_64::_rdtsc;
use std::time::Instant;

fn get_rdtsc() -> u64 {
    return unsafe { _rdtsc() };
}

pub fn get_freq_estimate(milliseconds_to_wait: u128) -> u128 {
    let cpu_start = get_rdtsc();
    let os_start = Instant::now();
    while os_start.elapsed().as_millis() < milliseconds_to_wait {}
    let os_elapsed = os_start.elapsed().as_millis();
    let cpu_stop = get_rdtsc();
    let cpu_elapsed = cpu_stop - cpu_start;
    (cpu_elapsed as u128) / os_elapsed
}
