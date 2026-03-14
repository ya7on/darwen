//! Criterion benchmark for the `RESTRICT` operator.

mod common;

use common::user_relation;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use darwen::prelude::{AttributeName, Predicate, Scalar};
use std::hint::black_box;
use std::time::Duration;

fn bench_restrict(c: &mut Criterion) {
    let predicate = Predicate::gt(AttributeName::from("age"), Scalar::Integer(40));
    let mut group = c.benchmark_group("restrict");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in [1_000_i64, 10_000, 50_000] {
        let relation = user_relation(size);

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let result = relation.restrict(black_box(&predicate)).unwrap();
                black_box(result);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_restrict);
criterion_main!(benches);
