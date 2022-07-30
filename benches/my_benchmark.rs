#![allow(unused)]

use compression::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::RngCore;

fn create_input() -> Vec<u8> {
    const LENGTH: usize = 1 << 15;
    let mut data = vec![0_u8; LENGTH];

    rand::thread_rng().fill_bytes(data.as_mut_slice());
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = create_input();
    c.bench_function("movetofront roundtrip", |b| {
        b.iter_batched(
            || data.clone(),
            |d| MoveToFront::decode(MoveToFront::encode(black_box(d)).unwrap()).unwrap(),
            BatchSize::LargeInput,
        )
    });
    c.bench_function("huffman roundtrip", |b| {
        b.iter_batched(
            || data.clone(),
            |d| Huffman::decode(Huffman::encode(black_box(d)).unwrap()).unwrap(),
            BatchSize::LargeInput,
        )
    });
    c.bench_function("bwt roundtrip", |b| {
        b.iter_batched(
            || data.clone(),
            |d| Bwt::decode(Bwt::encode(black_box(d)).unwrap()).unwrap(),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
