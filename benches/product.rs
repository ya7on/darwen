//! Criterion benchmark for the `PRODUCT` operator.

mod common;

use common::unary_integer_relation;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

fn bench_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("product");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in [25_i64, 50, 100] {
        let lhs = unary_integer_relation("left", 0, size);
        let rhs = unary_integer_relation("right", 0, size);

        group.throughput(Throughput::Elements((size * 2) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let result = lhs.product(black_box(&rhs)).unwrap();
                black_box(result);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_product);
criterion_main!(benches);
