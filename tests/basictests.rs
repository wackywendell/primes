use primes::{factors, factors_uniq, is_prime, PrimeSet, PrimeSetBasics, TrialDivision};

#[test]
fn test_primesetbasics() {
    let mut pset = TrialDivision::new();
    let ln = pset.list().len();
    pset.expand();

    assert_eq!(pset.list().len(), ln + 1);
}

#[test]
fn test_primeset() {
    let mut pset = TrialDivision::new();
    let (_idx, p) = pset.find(10);

    assert_eq!(p, 11);
}

#[test]
fn test_iter() {
    let mut pset = TrialDivision::new();
    let first_few = [2u64, 3, 5, 7, 11, 13, 17, 19, 23];
    for (m, &n) in pset.iter().zip(first_few.iter()) {
        assert_eq!(m, n);
    }
}

#[test]
fn test_find() {
    let mut pset = TrialDivision::new();

    // pset is empty, so it needs to generate the primes
    assert_eq!(pset.find_vec(1000), None);

    let (ix_exp, n_exp) = (168, 1009);

    assert_eq!(pset.find(1000), (ix_exp, n_exp));
    assert_eq!(pset.find(n_exp), (ix_exp, n_exp));

    // We shouldn't have gone beyond 1009
    {
        let plst = pset.list();
        let plen = plst.len();
        assert_eq!(plen, ix_exp + 1);

        assert_eq!(plst[plen - 1], n_exp);
    }

    assert_eq!(pset.find_vec(n_exp), Some((ix_exp, n_exp)));
}

#[test]
fn test_primes() {
    let mut pset = TrialDivision::new();

    // note: some are repeated, because the pset list grows as it goes

    assert!(!pset.is_prime(1));
    assert!(!is_prime(1));
    assert!(pset.is_prime(2));
    assert!(is_prime(2));
    assert!(pset.is_prime(13));
    assert!(is_prime(13));
    assert!(!pset.is_prime(45));
    assert!(!is_prime(45));
    assert!(!pset.is_prime(13 * 13));
    assert!(!is_prime(13 * 13));
    assert!(pset.is_prime(13));
    assert!(pset.is_prime(7));
    assert!(is_prime(7));
    assert!(!pset.is_prime(9));
    assert!(!is_prime(9));
    assert!(pset.is_prime(5));
    assert!(is_prime(5));
    assert!(pset.is_prime(954377));
    assert!(pset.is_prime(954379));
    assert!(!pset.is_prime(954377 * 954379));

    assert!(!is_prime(18409199 * 18409201));
    assert!(pset.is_prime(18409199));
    assert!(pset.is_prime(18409201));
    assert!(!pset.is_prime(2147483643));
    assert!(pset.is_prime(2147483647));
    assert!(!pset.is_prime(2147483649));
    assert!(!pset.is_prime(63061493));
    assert!(!pset.is_prime(63061491));
    assert!(pset.is_prime(63061489));
    assert!(!pset.is_prime(63061487));
    assert!(!pset.is_prime(63061485));
    assert!(pset.is_prime(2147483647));
    assert!(pset.is_prime(63061489));
    assert!(!is_prime(63061489 * 2147483647));
    assert!(!is_prime(63061489 * 63061489));
    // assert!(!is_prime(2147483647 * 2147483647)); // Runs very long
}

#[test]
fn test_factors() {
    let mut pset = TrialDivision::new();

    let ns = [
        (1, vec![]),
        (2, vec![2]),
        (3, vec![3]),
        (4, vec![2, 2]),
        (5, vec![5]),
        (6, vec![2, 3]),
        (9, vec![3, 3]),
        (12, vec![2, 2, 3]),
        (121, vec![11, 11]),
        (144, vec![2, 2, 2, 2, 3, 3]),
        (10_000_000, vec![2, 2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5]),
        (100, vec![2, 2, 5, 5]),
        (121, vec![11, 11]),
    ];

    // Test unique factors
    for &(n, ref v) in ns.iter() {
        println!("{}: {:?}", n, v);
        assert_eq!(pset.prime_factors(n), *v);
        assert_eq!(factors(n), *v);

        let unique_factors = factors_uniq(n);

        // Get unique factors from the lists we made above
        let mut unique_factors_exp: Vec<u64> = v.iter().map(|&x| x).collect();
        unique_factors_exp.dedup();

        assert_eq!(unique_factors, unique_factors_exp);
    }

    pset = TrialDivision::new();
    assert_eq!(pset.prime_factors(12), vec!(2, 2, 3));
}
