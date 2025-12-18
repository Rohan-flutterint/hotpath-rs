#[hotpath::measure]
fn noop() {}

#[hotpath::main]
fn main() {
    for _ in 0..1_000_000 {
        noop();
    }
}
