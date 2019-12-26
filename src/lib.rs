/*!

A basic library for finding primes, providing a basic Iterator over all primes. It is not as fast as
`slow_primes`, but it is meant to be easy to use!

The simplest usage is simply to create an `Iterator`:

```
use primes::PrimeSet;

let mut pset = PrimeSet::new();

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
use primes::PrimeSet;

let mut pset = PrimeSet::new();
let (ix, n) = pset.find(1_000_000);

println!("Prime {}: {}", ix, n);
```

# Example: Find the first ten primes *after* the thousandth prime
```
use primes::PrimeSet;

let mut pset = PrimeSet::new();
for (ix, n) in pset.iter().enumerate().skip(1_000).take(10) {
    println!("Prime {}: {}", ix, n);
}
```

# Example: Find the first prime greater than 1000
```
use primes::PrimeSet;

let mut pset = PrimeSet::new();
let (ix, n) = pset.find(1_000);
println!("The first prime after 1000 is the {}th prime: {}", ix, n);

assert_eq!(pset.find(n), (ix, n));
```

For more info on use, see `PrimeSet`, a class which handles the Sieve and has multiple methods for
iterating over primes.

This also provides a few functions unconnected to `PrimeSet`, which will be faster for the first
case, but slower in the long term as they do not use any caching of primes.

*/
#![doc(html_root_url = "https://wackywendell.github.io/primes/")]

use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Index;
use std::slice;

/** A prime generator, using the Sieve of Eratosthenes.

Create with `let mut pset = PrimeSet::new()`, and then use `pset.iter()` to iterate over all primes.
**/
#[derive(Default)]
pub struct PrimeSet {
    lst: Vec<u64>,
}

/// An iterator over generated primes. Created by `PrimeSet::iter` or
/// `PrimeSet::generator`
pub struct PrimeSetIter<'a> {
    p: &'a mut PrimeSet,
    n: usize,
    expand: bool,
}

impl PrimeSet {
    /// A new prime generator, primed with 2 and 3
    pub fn new() -> PrimeSet {
        PrimeSet { lst: vec![2, 3] }
    }

    /// Finds one more prime, and adds it to the list
    pub fn expand(&mut self) {
        let mut l: u64 = self.lst[self.lst.len() - 1] + 2;
        let mut remainder = 0;
        loop {
            for &n in &self.lst {
                remainder = l % n;
                if remainder == 0 || n * n > l {
                    break;
                }
            }

            if remainder != 0 {
                self.lst.push(l);
                break;
            };

            l += 2;
        }
    }

    /// Number of primes found so far
    pub fn len(&self) -> usize {
        self.lst.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    /// Return all primes found so far as a slice
    pub fn list(&self) -> &[u64] {
        &self.lst[..]
    }

    /// Iterator over all primes not yet found
    pub fn generator(&mut self) -> PrimeSetIter {
        let myn = self.len();
        PrimeSetIter {
            p: self,
            n: myn,
            expand: true,
        }
    }

    /// Iterator over all primes, starting with 2. If you don't care about the "state" of the
    /// `PrimeSet`, this is what you want!
    pub fn iter(&mut self) -> PrimeSetIter {
        PrimeSetIter {
            p: self,
            n: 0,
            expand: true,
        }
    }

    /// Iterator over just the primes found so far
    pub fn iter_vec(&self) -> slice::Iter<u64> {
        self.lst.iter()
    }

    /// Find the next largest prime from a number
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
    pub fn find(&mut self, n: u64) -> (usize, u64) {
        while n > *(self.lst.last().unwrap_or(&0)) {
            self.expand();
        }
        self.find_vec(n).unwrap()
    }

    /// Check if a number is prime
    ///
    /// Note that this only requires primes up to `n.sqrt()` to be generated, and will generate
    /// them as necessary on its own.
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
    pub fn is_prime(&mut self, n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 {
            return true;
        } // otherwise we get 2 % 2 == 0!
        for m in self.iter() {
            if n % m == 0 {
                return false;
            };
            if m * m > n {
                return true;
            };
        }
        unreachable!("This iterator should not be empty.");
    }

    /// Find the next largest prime from a number, if it is within the already-found list
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
    pub fn find_vec(&self, n: u64) -> Option<(usize, u64)> {
        if n > *(self.lst.last().unwrap_or(&0)) {
            return None;
        }

        let mut base: usize = 0;
        let mut lim: usize = self.len();

        // Binary search algorithm
        while lim != 0 {
            let ix = base + (lim >> 1);
            match self.lst[ix].cmp(&n) {
                Equal => return Some((ix, self.lst[ix])),
                Less => {
                    base = ix + 1;
                    lim -= 1;
                }
                Greater => (),
            }
            lim >>= 1;
        }
        Some((base, self.lst[base]))
    }

    /// Get the nth prime, even if we haven't yet found it
    pub fn get(&mut self, index: usize) -> u64 {
        for _ in 0..(index as isize) + 1 - (self.lst.len() as isize) {
            self.expand();
        }
        self.lst[index]
    }

    /// Get the prime factors of a number, starting from 2, including repeats
    pub fn prime_factors(&mut self, n: u64) -> Vec<u64> {
        if n == 1 {
            return Vec::new();
        }
        let mut curn = n;
        let mut lst: Vec<u64> = Vec::new();
        for p in self.iter() {
            while curn % p == 0 {
                lst.push(p);
                curn /= p;
                if curn == 1 {
                    return lst;
                }
            }

            if p * p > curn {
                lst.push(curn);
                return lst;
            }
        }
        unreachable!("This should be unreachable.");
    }
}

impl Index<usize> for PrimeSet {
    type Output = u64;
    fn index(&self, index: usize) -> &u64 {
        &self.lst[index]
    }
}

impl<'a> Iterator for PrimeSetIter<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        while self.n >= self.p.len() {
            if self.expand {
                self.p.expand()
            } else {
                return None;
            }
        }
        self.n += 1;

        let m = self.p.lst[self.n - 1];

        Some(m)
    }
}

/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
    // TODO: return to step_by
    // for n in (3..).step_by(2).take_while(|m| m*m <= x) {
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    // No factor found. It must be prime.
    x
}

/// Find all prime factors of a number
/// Does not use a `PrimeSet`, but simply counts upwards
pub fn factors(x: u64) -> Vec<u64> {
    if x <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop {
        let m = firstfac(curn);
        lst.push(m);
        if m == curn {
            break;
        } else {
            curn /= m
        };
    }
    lst
}

/// Find all unique prime factors of a number
pub fn factors_uniq(x: u64) -> Vec<u64> {
    if x <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop {
        let m = firstfac(curn);
        lst.push(m);
        if curn == m {
            break;
        }
        while curn % m == 0 {
            curn /= m;
        }
        if curn == 1 {
            break;
        }
    }
    lst
}

/// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}
