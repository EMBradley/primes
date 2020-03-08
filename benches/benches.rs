use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use primes::PrimeSet;

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimeSet::find");

    let mut sizes: Vec<u64> = Vec::new();
    for &base in &[5_000, 50_000] {
        for size in 2..=20 {
            sizes.push(base / 2 * size);
        }
    }
    sizes.sort();
    sizes.dedup();

    for &size in sizes.iter() {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut pset = PrimeSet::new();
                black_box(pset.find(size))
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_primes);
criterion_main!(benches);
