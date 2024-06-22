use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use primes::{factors, PrimeSet, Sieve, TrialDivision};
use rand::prelude::*;

fn bench_primes(c: &mut Criterion) {
    let mut sizes: Vec<u64> = Vec::new();
    for base in [5_000, 50_000] {
        for size in 2..=20 {
            sizes.push(base / 2 * size);
        }
    }
    sizes.sort();
    sizes.dedup();

    let mut group = c.benchmark_group("Find");
    for size in sizes {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(
            BenchmarkId::new("TrialDivision", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut pset = TrialDivision::new();
                    black_box(pset.find(size))
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("Sieve", size), &size, |b, &size| {
            b.iter(|| {
                let mut pset = Sieve::new();
                black_box(pset.find(size))
            })
        });
    }
    group.finish();
}

fn bench_factors(c: &mut Criterion) {
    let mut group = c.benchmark_group("Factors");
    for size in 1..=16 {
        group.bench_with_input(
            BenchmarkId::new("Factors", 10u64.pow(size)),
            &size,
            |b, &size| {
                let mut rng = thread_rng();
                b.iter(|| {
                    let n = rng.gen_range(10u64.pow(size)..10u64.pow(size + 1));
                    black_box(factors(n))
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_primes, bench_factors);
criterion_main!(benches);
