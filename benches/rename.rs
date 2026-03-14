//! Criterion benchmark for the `RENAME` operator.

mod common;

use common::user_relation;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use darwen::prelude::AttributeName;
use std::hint::black_box;
use std::time::Duration;

fn bench_rename(c: &mut Criterion) {
    let mapping = vec![
        (AttributeName::from("id"), AttributeName::from("user_id")),
        (AttributeName::from("age"), AttributeName::from("years")),
    ];
    let mut group = c.benchmark_group("rename");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for size in [1_000_i64, 10_000, 50_000] {
        let relation = user_relation(size);

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let result = relation.rename(black_box(&mapping)).unwrap();
                black_box(result);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_rename);
criterion_main!(benches);
