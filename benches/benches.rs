use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use primes::PrimeSet;

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimeSet::find");
    for size in [
        100, 200, 500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 100_000, 200_000, 500_000,
    ]
    .iter()
    {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
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
