/*!

A basic library for finding primes, providing a basic Iterator over all primes. It is not as fast as
`slow_primes`, but it is meant to be easy to use!

The simplest usage is simply to create an `Iterator`:

```
use primes::{Sieve, PrimeSet};

let mut pset = Sieve::new();

for (ix, n) in pset.iter().enumerate().take(10) {
    println!("Prime {}: {}", ix, n);
}
```

This library provides methods for generating primes, testing whether a number is prime, and
factorizing numbers. Most methods generate primes lazily, so only enough primes will be generated
for the given test, and primes are cached for later use.

[*Source*](https://github.com/wackywendell/primes)

# Example: Find the first prime after 1 million

```
use primes::{Sieve, PrimeSet};

let mut pset = Sieve::new();
let (ix, n) = pset.find(1_000_000);

println!("Prime {}: {}", ix, n);
```

# Example: Find the first ten primes *after* the thousandth prime
```
use primes::{Sieve, PrimeSet};

let mut pset = Sieve::new();
for (ix, n) in pset.iter().enumerate().skip(1_000).take(10) {
    println!("Prime {}: {}", ix, n);
}
```

# Example: Find the first prime greater than 1000
```
use primes::{Sieve, PrimeSet};

let mut pset = Sieve::new();
let (ix, n) = pset.find(1_000);
println!("The first prime after 1000 is the {}th prime: {}", ix, n);

assert_eq!(pset.find(n), (ix, n));
```

For more info on use, see `PrimeSet`, a class which encapsulates most of the functionality and has
multiple methods for iterating over primes.

This also provides a few functions unconnected to `PrimeSet`, which will be faster for the first
case, but slower in the long term as they do not use any caching of primes.

*/
#![doc(html_root_url = "https://wackywendell.github.io/primes/")]

use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::slice;

pub trait PrimeSetBasics {
    /// Finds one more prime, and adds it to the list
    fn expand(&mut self);

    /// Return all primes found so far as a slice
    fn list(&self) -> &[u64];
}

const FIRST_PRIMES: [u64; 3] = [2, 3, 5];
const WHEEL_LENGTH: usize = 8;
const WHEEL_CIRCUMFERENCE: u64 = WHEEL_235[WHEEL_LENGTH - 1] + 1;
const WHEEL_235: [u64; WHEEL_LENGTH] = [1, 7, 11, 13, 17, 19, 23, 29];

#[derive(Copy, Clone)]
struct Wheel {
    offset: u64,
    index: usize,
    factor: u64,
}

/**
A prime generator, using the Trial Division method.

Create with `let mut pset = TrialDivision::new()`, and then use `pset.iter()` to iterate over all
primes.
**/
#[derive(Clone)]
pub struct TrialDivision {
    list: Vec<u64>,
}

/**
A prime generator, using the Sieve of Eratosthenes method. This is asymptotically more efficient
than the Trial Division method, but slower earlier on.

Create with `let mut pset = Sieve::new()`, and then use `pset.iter()` to iterate over all primes.
**/
#[derive(Clone)]
pub struct Sieve {
    primes: Vec<u64>,
    wheel: Wheel,
    sieve: BinaryHeap<Reverse<Wheel>>,
}

/// An iterator over generated primes. Created by `PrimeSet::iter` or
/// `PrimeSet::generator`
pub struct PrimeSetIter<'a, P: PrimeSet> {
    p: &'a mut P,
    n: usize,
    expand: bool,
}

impl Wheel {
    fn new() -> Self {
        Self {
            offset: 0,
            index: 0,
            factor: 1,
        }
    }

    fn get_value(&self) -> u64 {
        self.factor * (self.offset + WHEEL_235[self.index])
    }
}

impl Iterator for Wheel {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.index + 1 >= WHEEL_LENGTH {
            self.index = 0;
            self.offset += WHEEL_CIRCUMFERENCE;
        } else {
            self.index += 1;
        }

        Some(self.get_value())
    }
}

impl Default for Wheel {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Wheel {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().eq(&other.get_value())
    }
}

impl Eq for Wheel {}

impl Ord for Wheel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl PartialOrd for Wheel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TrialDivision {
    /// A new prime generator, primed with 2 and 3
    pub fn new() -> TrialDivision {
        TrialDivision { list: vec![2, 3] }
    }
}

impl Default for TrialDivision {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimeSetBasics for TrialDivision {
    /// Finds one more prime, and adds it to the list
    fn expand(&mut self) {
        let last_prime = self.list.last().unwrap();
        let next_prime = (last_prime + 2..)
            .step_by(2)
            .find(|&n| {
                self.list()
                    .iter()
                    .take_while(|&p| p * p <= n)
                    .all(|&p| n % p > 0)
            })
            .unwrap();

        self.list.push(next_prime);
    }

    /// Return all primes found so far as a slice
    fn list(&self) -> &[u64] {
        &self.list[..]
    }
}

impl Default for Sieve {
    fn default() -> Self {
        Self::new()
    }
}

impl Sieve {
    pub fn new() -> Self {
        Self::from_seed(FIRST_PRIMES.to_vec(), Wheel::new())
    }

    fn from_seed(seed_primes: Vec<u64>, wheel: Wheel) -> Self {
        Sieve {
            primes: seed_primes,
            sieve: BinaryHeap::new(),
            wheel,
        }
    }

    // insert a prime and its composite. If the composite is already occupied, we'll increase
    // the composite by prime and put it there, repeating as necessary.
    fn insert(&mut self, prime: u64) {
        let mut wheel = self.wheel;
        wheel.factor = prime;
        self.primes.push(prime);
        self.sieve.push(Reverse(wheel));
    }

    fn adjust_sieve(&mut self, n: u64) {
        while let Some(mut wheel) = self.sieve.peek_mut() {
            if wheel.0.get_value() > n {
                break;
            }
            let _ = wheel.0.next();
        }
    }
}

impl PrimeSetBasics for Sieve {
    /// Finds one more prime, and adds it to the list
    fn expand(&mut self) {
        loop {
            let n = self.wheel.next().unwrap();

            let Some(wheel) = self.sieve.peek() else {
                self.insert(n);
                return;
            };

            if wheel.0.get_value() > n {
                self.insert(n);
                return;
            }

            self.adjust_sieve(n);
        }
    }

    /// Return all primes found so far as a slice
    fn list(&self) -> &[u64] {
        &self.primes[..]
    }
}

pub trait PrimeSet: PrimeSetBasics + Sized {
    /// Number of primes found so far
    fn len(&self) -> usize {
        self.list().len()
    }

    fn is_empty(&self) -> bool {
        self.list().is_empty()
    }

    /// Iterator over all primes not yet found
    fn generator(&mut self) -> PrimeSetIter<Self> {
        let myn = self.len();
        PrimeSetIter {
            p: self,
            n: myn,
            expand: true,
        }
    }

    /// Iterator over all primes, starting with 2. If you don't care about the "state" of the
    /// `PrimeSet`, this is what you want!
    fn iter(&mut self) -> PrimeSetIter<Self> {
        PrimeSetIter {
            p: self,
            n: 0,
            expand: true,
        }
    }

    /// Iterator over just the primes found so far
    fn iter_vec(&self) -> slice::Iter<u64> {
        self.list().iter()
    }

    /// Find the next largest prime from a number
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
    fn find(&mut self, n: u64) -> (usize, u64) {
        loop {
            match self.list().last() {
                None => self.expand(),
                Some(&p) if p < n => self.expand(),
                _ => return self.find_vec(n).unwrap(),
            }
        }
    }

    /// Check if a number is prime
    ///
    /// Note that this only requires primes up to `n.sqrt()` to be generated, and will generate
    /// them as necessary on its own.
    fn is_prime(&mut self, n: u64) -> bool {
        if n <= 1 {
            return false;
        }

        self.iter().take_while(|&m| m * m <= n).all(|m| n % m != 0)
    }

    /// Find the next largest prime from a number, if it is within the already-found list
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
    fn find_vec(&self, n: u64) -> Option<(usize, u64)> {
        if n > *(self.list().last().unwrap_or(&0)) {
            return None;
        }

        let mut start = 0;
        let mut end = self.len();

        while start < end {
            let midpoint = start + (end - start) / 2;
            let mid_value = self.list().get(midpoint)?;

            match mid_value.cmp(&n) {
                Equal => return Some((midpoint, *mid_value)),
                Less => {
                    start = midpoint + 1;
                }
                Greater => {
                    end = midpoint;
                }
            }
        }

        Some((start, self.list()[start]))
    }

    /// Get the nth prime, even if we haven't yet found it
    fn get(&mut self, index: usize) -> u64 {
        while self.list().len() < index + 1 {
            self.expand();
        }
        self.list()[index]
    }

    /// Get the prime factors of a number, starting from 2, including repeats
    fn prime_factors(&mut self, mut n: u64) -> Vec<u64> {
        let mut factors = Vec::new();

        for p in self.iter() {
            while n % p == 0 {
                factors.push(p);
                n /= p;
            }

            if n < 2 {
                return factors;
            }

            if p * p > n {
                factors.push(n);
                return factors;
            }
        }
        unreachable!();
    }
}

impl<P: PrimeSetBasics> PrimeSet for P {}

impl Index<usize> for TrialDivision {
    type Output = u64;
    fn index(&self, index: usize) -> &u64 {
        &self.list()[index]
    }
}

impl<'a, P: PrimeSet> Iterator for PrimeSetIter<'a, P> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        while self.n >= self.p.list().len() {
            if self.expand {
                self.p.expand();
            } else {
                return None;
            }
        }
        self.n += 1;

        let m = self.p.list()[self.n - 1];

        Some(m)
    }
}

/// Find the first prime factor of a number
fn first_factor(x: u64) -> u64 {
    let factors_to_check = FIRST_PRIMES.iter().copied().chain(Wheel::default());
    factors_to_check
        .take_while(|&n| n * n <= x)
        .find(|&n| x % n == 0)
        .unwrap_or(x)
}

/// Find all prime factors of a number
/// Does not use a `PrimeSet`, but simply counts upwards
pub fn factors(mut x: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    while x > 1 {
        let factor = first_factor(x);
        while x % factor == 0 {
            factors.push(factor);
            x /= factor;
        }
    }

    factors
}

/// Find all unique prime factors of a number
pub fn factors_uniq(mut x: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    while x > 1 {
        let factor = first_factor(x);
        while x % factor == 0 {
            x /= factor;
        }
        factors.push(factor);
    }

    factors
}

/// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    first_factor(n) == n
}
