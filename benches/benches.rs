#![feature(test)]

extern crate primes;
extern crate test;
use primes::PrimeSet;

use test::Bencher;

#[bench]
fn bench_primes(b: &mut Bencher) {
	b.iter(|| {
		let mut pset = PrimeSet::new();
		let (_, _) = pset.find(1_000_000);
		//~ let (idx, n) = pset.find(1_000_000);
		//~ println!("Prime {}: {}", idx, n);
	})
}
