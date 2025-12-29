use criterion::{criterion_group, criterion_main, Criterion};

fn bench_placeholder(c: &mut Criterion) {
    c.bench_function("placeholder", |b| b.iter(|| {
        // Replace with actual benchmark (e.g., plan_operation)
        let mut v = Vec::with_capacity(100);
        for i in 0..100 { v.push(i); }
        v.sort();
    }));
}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
