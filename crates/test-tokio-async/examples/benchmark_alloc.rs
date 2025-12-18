#[hotpath::measure]
fn alloc() {
    let vec = vec![1u8; 1024];
    std::hint::black_box(vec);
}

#[hotpath::main]
fn main() {
    for _ in 0..1_000_000 {
        alloc();
    }
}
