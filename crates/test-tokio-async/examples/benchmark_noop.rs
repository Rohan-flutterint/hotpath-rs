use std::time::{Duration, Instant};

const WAIT_FOR: Duration = Duration::from_micros(1);

// This function is used to spin-wait for a given duration.
// It's more precise than std::thread::sleep.
#[hotpath::measure]
fn spin_wait() {
    let start = Instant::now();
    while start.elapsed() < WAIT_FOR {}
}

#[hotpath::main]
fn main() {
    for _ in 0..100_000 {
        spin_wait();
    }
}
