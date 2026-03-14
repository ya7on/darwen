//! Criterion benchmark for the `INTERSECT` operator.

mod common;

use common::unary_integer_relation;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

fn bench_intersect(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersect");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in [1_000_i64, 5_000, 20_000] {
        let lhs = unary_integer_relation("value", 0, size);
        let rhs = unary_integer_relation("value", size / 2, size);

        group.throughput(Throughput::Elements((size * 2) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let result = lhs.intersect(black_box(&rhs)).unwrap();
                black_box(result);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_intersect);
criterion_main!(benches);
