use criterion::{criterion_group, criterion_main, Criterion};

use palette_extractor::k_means;

fn bench(c: &mut Criterion) {
    const K: usize = 6;

    let image = image::open("benches/bart.png").unwrap();

    c.bench_function("sync", |b| {
        b.iter(|| {
            k_means::k_cluster(&image, K).unwrap();
        })
    });
}

criterion_group! {
    name = benches;

    config = Criterion::default();

    targets = bench
}

criterion_main!(benches);
