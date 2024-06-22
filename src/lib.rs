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

const FIRST_PRIMES: [u64; 5] = [2, 3, 5, 7, 11];
const WHEEL_LENGTH: usize = 480;
const WHEEL_CIRCUMFERENCE: u64 = WHEEL_11[WHEEL_LENGTH - 1] + 1;
const WHEEL_11: [u64; WHEEL_LENGTH] = [
    1, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
    107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 169, 173, 179, 181, 191, 193, 197,
    199, 211, 221, 223, 227, 229, 233, 239, 241, 247, 251, 257, 263, 269, 271, 277, 281, 283, 289,
    293, 299, 307, 311, 313, 317, 323, 331, 337, 347, 349, 353, 359, 361, 367, 373, 377, 379, 383,
    389, 391, 397, 401, 403, 409, 419, 421, 431, 433, 437, 439, 443, 449, 457, 461, 463, 467, 479,
    481, 487, 491, 493, 499, 503, 509, 521, 523, 527, 529, 533, 541, 547, 551, 557, 559, 563, 569,
    571, 577, 587, 589, 593, 599, 601, 607, 611, 613, 617, 619, 629, 631, 641, 643, 647, 653, 659,
    661, 667, 673, 677, 683, 689, 691, 697, 701, 703, 709, 713, 719, 727, 731, 733, 739, 743, 751,
    757, 761, 767, 769, 773, 779, 787, 793, 797, 799, 809, 811, 817, 821, 823, 827, 829, 839, 841,
    851, 853, 857, 859, 863, 871, 877, 881, 883, 887, 893, 899, 901, 907, 911, 919, 923, 929, 937,
    941, 943, 947, 949, 953, 961, 967, 971, 977, 983, 989, 991, 997, 1003, 1007, 1009, 1013, 1019,
    1021, 1027, 1031, 1033, 1037, 1039, 1049, 1051, 1061, 1063, 1069, 1073, 1079, 1081, 1087, 1091,
    1093, 1097, 1103, 1109, 1117, 1121, 1123, 1129, 1139, 1147, 1151, 1153, 1157, 1159, 1163, 1171,
    1181, 1187, 1189, 1193, 1201, 1207, 1213, 1217, 1219, 1223, 1229, 1231, 1237, 1241, 1247, 1249,
    1259, 1261, 1271, 1273, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1313, 1319, 1321,
    1327, 1333, 1339, 1343, 1349, 1357, 1361, 1363, 1367, 1369, 1373, 1381, 1387, 1391, 1399, 1403,
    1409, 1411, 1417, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1457, 1459, 1469, 1471, 1481,
    1483, 1487, 1489, 1493, 1499, 1501, 1511, 1513, 1517, 1523, 1531, 1537, 1541, 1543, 1549, 1553,
    1559, 1567, 1571, 1577, 1579, 1583, 1591, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1633,
    1637, 1643, 1649, 1651, 1657, 1663, 1667, 1669, 1679, 1681, 1691, 1693, 1697, 1699, 1703, 1709,
    1711, 1717, 1721, 1723, 1733, 1739, 1741, 1747, 1751, 1753, 1759, 1763, 1769, 1777, 1781, 1783,
    1787, 1789, 1801, 1807, 1811, 1817, 1819, 1823, 1829, 1831, 1843, 1847, 1849, 1853, 1861, 1867,
    1871, 1873, 1877, 1879, 1889, 1891, 1901, 1907, 1909, 1913, 1919, 1921, 1927, 1931, 1933, 1937,
    1943, 1949, 1951, 1957, 1961, 1963, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011, 2017, 2021,
    2027, 2029, 2033, 2039, 2041, 2047, 2053, 2059, 2063, 2069, 2071, 2077, 2081, 2083, 2087, 2089,
    2099, 2111, 2113, 2117, 2119, 2129, 2131, 2137, 2141, 2143, 2147, 2153, 2159, 2161, 2171, 2173,
    2179, 2183, 2197, 2201, 2203, 2207, 2209, 2213, 2221, 2227, 2231, 2237, 2239, 2243, 2249, 2251,
    2257, 2263, 2267, 2269, 2273, 2279, 2281, 2287, 2291, 2293, 2297, 2309,
];

#[derive(Copy, Clone)]
struct Wheel {
    offset: u64,
    index: usize,
    factor: u64,
    value: u64,
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
            value: 0,
        }
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

        self.value = self.factor * (self.offset + WHEEL_11[self.index]);

        Some(self.value)
    }
}

impl Default for Wheel {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Wheel {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for Wheel {}

impl Ord for Wheel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
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
        wheel.value = prime * prime;
        self.primes.push(prime);
        self.sieve.push(Reverse(wheel));
    }

    fn adjust_sieve(&mut self, n: u64) {
        while let Some(mut wheel) = self.sieve.peek_mut() {
            if wheel.0.value > n {
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

            if wheel.0.value > n {
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
