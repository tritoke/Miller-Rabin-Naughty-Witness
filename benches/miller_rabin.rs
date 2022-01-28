use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"]
mod main;
use main::miller_rabin;

fn bench_miller_rabin_raw(c: &mut Criterion) {
    c.bench_function("miller_rabin 69 100", |b| b.iter(|| miller_rabin(black_box(69), black_box(100))));
    c.bench_function("miller_rabin 112412423412 2345234534544", |b| b.iter(|| miller_rabin(black_box(112412423411), black_box(2345234534545))));
}

criterion_group!(benches, bench_miller_rabin_raw);
criterion_main!(benches);
