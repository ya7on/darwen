//! Criterion benchmark for the natural `JOIN` operator.

mod common;

use common::keyed_integer_relation;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

fn bench_join(c: &mut Criterion) {
    let mut group = c.benchmark_group("join");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in [100_i64, 500, 1_000] {
        let lhs = keyed_integer_relation("left_value", 0, size, 10);
        let rhs = keyed_integer_relation("right_value", size / 2, size, 100);

        group.throughput(Throughput::Elements((size * 2) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let relation = lhs.join(black_box(&rhs)).unwrap();
                black_box(relation);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_join);
criterion_main!(benches);
