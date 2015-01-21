/*!
A basic library for finding primes, using the Sieve of Eratosthenes. This library provides methods
for generating primes, testing whether a number is prime, and factorizing numbers. Most methods
generate primes lazily, so only enough primes will be generated for the given test, and primes are
cached for later use.

To use, see `PrimeSet`, a class which handles the Sieve and has multiple methods for iterating
over primes.

This also provides a few functions unconnected to `PrimeSet`, which will be faster for the first case,
but slower in the long term as they do not use any caching of primes.
*/

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

#[cfg(test)]
extern crate test;

use std::ops::Index;
use std::slice;
use std::iter;
use std::num::{Float,cast};
use std::cmp::Ordering::{Equal,Less,Greater};

#[cfg(test)]
use test::Bencher;

fn sqrt_floor<T: std::num::NumCast>(n : T) -> T {
    cast::<f64, T>(
        (cast::<T, f64>(n).unwrap()).sqrt().floor()
    ).unwrap()
}

fn sqrt_ceil<T: std::num::NumCast>(n : T) -> T {
    cast::<f64, T>(
        (cast::<T, f64>(n).unwrap()).sqrt().ceil()
    ).unwrap()
}

/** A prime generator, using the Sieve of Eratosthenes.

Create with `let mut pset = PrimeSet::new()`, and then use `pset.iter()` to iterate over all primes.
**/
pub struct PrimeSet {
    lst : Vec<u64>
}

/// An iterator over generated primes. Created by PrimeSet::iter or
/// PrimeSet::generator
pub struct PrimeSetIter<'a> {
    p : &'a mut PrimeSet,
    n : usize,
    expand : bool
}

impl PrimeSet {
    /// A new prime generator, primed with 2 and 3
    pub fn new() -> PrimeSet {
        PrimeSet{lst:vec!(2,3)}
    }
    
    /// Finds one more prime, and adds it to the list
    pub fn expand(&mut self) {
        let mut l : u64 = self.lst[self.lst.len()-1] + 2;
        let mut sql = sqrt_floor(l);
        let mut remainder = 0;
        loop {
            for &n in self.lst.iter() {
                remainder = l % n;
                if remainder == 0 || n > sql {
                    break;
                }
            };
            
            if remainder != 0 {
                self.lst.push(l);
                break;
            };
            
            l += 2;
            sql = sqrt_floor(l);
        }
    }
    
    /// Number of primes found so far
    pub fn len(&self) -> usize {
        self.lst.len()
    }
    
    /// Return all primes found so far as a slice
    pub fn list<'a>(&'a self) -> &'a [u64] {
        self.lst.as_slice()
    }
    
    /// Iterator over all primes not yet found
    pub fn generator<'a>(&'a mut self) -> PrimeSetIter<'a> {
        let myn = self.len();
        PrimeSetIter{p:self, n:myn, expand:true}
    }
    
    /// Iterator over all primes, starting with 2. If you don't care about the "state" of the 
    /// PrimeSet, this is what you want!
    pub fn iter<'a>(&'a mut self) -> PrimeSetIter<'a> {
        PrimeSetIter{p:self, n:0, expand:true}
    }
    
    //~ pub fn iter_once(&'self mut self) -> PrimeSetIter<'self> {
        //~ PrimeSetIter{p:self, n:0, expand:false}
    //~ }
    
    /// Iterator over just the primes found so far
    pub fn iter_vec<'a>(&'a self) -> slice::Iter<'a, u64> {
        self.lst.iter()
    }
    
    /// Find the next largest prime from a number
    /// Returns (idx, prime)
    /// Note that if n is prime, then the output will be (idx, n)
    pub fn find(&mut self, n: u64) -> (usize, u64) {
        while n > *(self.lst.last().unwrap_or(&0)){
            self.expand();
        }
        self.find_vec(n).unwrap()
    }
    
    /// Check if a number is prime
    /// Note that this only requires primes up to n.sqrt() to be generated, and will generate
    /// them as necessary on its own.
    pub fn is_prime(&mut self, n: u64) -> bool {
        if n <= 1 {return false;}
        if n == 2 {return true;} // otherwise we get 2 % 2 == 0!
        for m in self.iter() {
            if n % m == 0 {return false;};
            if m*m > n {return true;};
        }
        panic!("This iterator should not be empty.");
    }
    
    /// Find the next largest prime from a number, if it is within the already-found list
    /// Returns (idx, prime)
    /// Note that if n is prime, then the output will be (idx, n)
    pub fn find_vec(&self, n: u64) -> Option<(usize, u64)> {
        if n > *(self.lst.last().unwrap_or(&0)){ return None;}
        
        let mut base : usize = 0;
        let mut lim : usize = self.len();

        // Binary search algorithm
        while lim != 0 {
            let ix = base + (lim >> 1);
            match self.lst[ix].cmp(&n) {
                Equal => return Some((ix, self.lst[ix])),
                Less => {
                    base = ix + 1;
                    lim -= 1;
                }
                Greater => ()
            }
            lim >>= 1;
        }
        return Some((base, self.lst[base]));
    }
    
    /// Get the nth prime, even if we haven't yet found it
    pub fn get(&mut self, index : &usize) -> &u64 {
		for _ in range(0, (*index as isize) + 1 - (self.lst.len() as isize)){
			self.expand();
		}
        self.lst.index(index)
	}
	
	/// Get the prime factors of a number, starting from 2, including repeats
	pub fn prime_factors(&mut self, n: u64) -> Vec<u64> {
		if n == 1 {return Vec::new();}
		let mut curn = n;	
		let mut m = sqrt_ceil(curn);
		let mut lst: Vec<u64> = Vec::new();
		for p in self.iter() {
			while curn % p == 0 {
				lst.push(p);
				curn /= p;
				if curn == 1 {return lst;}
				m = sqrt_ceil(curn);
			}
			
			if p > m {
				lst.push(p);
				return lst;
			}
		}
		panic!("This should be unreachable.");
	}
}

impl Index<usize> for PrimeSet {
    type Output = u64;
    fn index(&self, index: &usize) -> &u64 {
        self.lst.index(index)
    }
}

impl<'a> Iterator for PrimeSetIter<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        while self.n >= self.p.len(){
            match self.expand {
                true => self.p.expand(),
                false => return None
            }
        }
        self.n += 1;
        
        let m = self.p.lst[self.n-1];
        
        Some(m)
    }
}

/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    let m = sqrt_ceil(x);
    if x % 2 == 0 { return 2; };
    for n in iter::range_step(3, m + 1, 2) {
        if x % n == 0 { return n; };
    }
    return x;
}

/// Find all prime factors of a number
/// Does not use a PrimeSet, but simply counts upwards
pub fn factors(x: u64) -> Vec<u64> {
    if x <= 1 {return vec!()};
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop  {
        let m = firstfac(curn);
        lst.push(m);
        if m == curn { break  } else { curn /= m };
    }
    return lst
}

/// Find all unique prime factors of a number
pub fn factors_uniq(x: u64) -> Vec<u64> {
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop  {
        let m = firstfac(curn);
        lst.push(m);
        if curn == m { break ; }
        while curn % m == 0 { curn /= m; }
        if curn == 1 { break ; }
    }
    return lst
}

/// Test whether a number is prime. Checks every odd number up to sqrt(n).
pub fn is_prime(n : u64) -> bool {
    if n <= 1 {return false;}
    firstfac(n) == n
}

#[test]
fn test_iter(){
    let mut pset = PrimeSet::new();
    let first_few = [2u64,3,5,7,11,13,17,19,23];
    for (m, &n) in pset.iter().zip(first_few.iter()) {
        assert_eq!(m, n);
    }
}

#[test]
fn test_primes(){
    let mut pset = PrimeSet::new();
    
    // note: some are repeated, because the pset list grows as it goes
    
    assert!(!pset.is_prime(1));
    assert!(!is_prime(1));
    assert!(pset.is_prime(2));
    assert!(is_prime(2));
    assert!(pset.is_prime(13));
    assert!(is_prime(13));
    assert!(!pset.is_prime(45));
    assert!(!is_prime(45));
    assert!(!pset.is_prime(13*13));
    assert!(!is_prime(13*13));
    assert!(pset.is_prime(13));
    assert!(pset.is_prime(7));
    assert!(is_prime(7));
    assert!(!pset.is_prime(9));
    assert!(!is_prime(9));
    assert!(pset.is_prime(5));
    assert!(is_prime(5));
}

#[test]
fn test_factors(){
	let mut pset = PrimeSet::new();
	
	let ns = [  (1, vec!()),
                (2, vec!(2)),
                (3, vec!(3)),
                (4, vec!(2,2)),
                (5, vec!(5)),
                (6, vec!(2,3)),
	            (9, vec!(3,3)),
	            (12, vec!(2,2,3)),
	            (121, vec!(11,11)),
	            (144, vec!(2,2,2,2,3,3)),
	            (10_000_000, vec!(2,2,2,2,2,2,2,5,5,5,5,5,5,5)),
                (100, vec!(2,2,5,5)),
                (121, vec!(11, 11)),
                ];
    
    for &(n, ref v) in ns.iter(){
        assert_eq!(pset.prime_factors(n), *v);
        assert_eq!(factors(n), *v);
    }
	
	pset = PrimeSet::new();
	assert_eq!(pset.prime_factors(12), vec!(2,2,3));
	
}


#[bench]
fn bench_primes(b : &mut Bencher){
	b.iter(|| {
		let mut pset = PrimeSet::new();
		 let (_, _) = pset.find(1_000_000);
		 //~ let (idx, n) = pset.find(1_000_000);
		 //~ println!("Prime {}: {}", idx, n);
		 })
}
