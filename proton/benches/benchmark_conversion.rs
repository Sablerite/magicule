use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proton::convert_inches_to_meters;

fn benchmark_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("inch_to_meter_conversion");

    // Test various input values
    group.bench_function("zero", |b| {
        b.iter(|| convert_inches_to_meters(black_box(0.0f32)).unwrap())
    });

    group.bench_function("one", |b| {
        b.iter(|| convert_inches_to_meters(black_box(1.0f32)).unwrap())
    });

    group.bench_function("typical", |b| {
        b.iter(|| convert_inches_to_meters(black_box(39.3701f32)).unwrap())
    });

    group.bench_function("large", |b| {
        b.iter(|| convert_inches_to_meters(black_box(1000.0f32)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, benchmark_conversion);
criterion_main!(benches);
