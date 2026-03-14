//! Criterion benchmark for the `DIVIDE` operator.

mod common;

use common::{enrollment_relation, required_courses};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

fn bench_divide(c: &mut Criterion) {
    let course_count = 8_i64;
    let divisor = required_courses(course_count);
    let mut group = c.benchmark_group("divide");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for students in [100_i64, 500, 1_000] {
        let dividend = enrollment_relation(students, course_count);

        group.throughput(Throughput::Elements((students * course_count) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(students),
            &students,
            |b, &_students| {
                b.iter(|| {
                    let result = dividend.divide(black_box(&divisor)).unwrap();
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_divide);
criterion_main!(benches);
