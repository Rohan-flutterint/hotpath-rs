use std::time::{Duration, Instant};

const WAIT_FOR: Duration = Duration::from_micros(1);

// This function is used to spin-wait for a given duration.
// It's more precise than std::thread::sleep.
fn spin_wait() {
    let start = Instant::now();
    while start.elapsed() < WAIT_FOR {}
}

#[hotpath::measure]
fn alloc() {
    let vec = vec![1u8; 128];
    std::hint::black_box(vec);
    spin_wait();
}

#[hotpath::main]
fn main() {
    for _ in 0..100_000 {
        alloc();
    }
}
