/*
 * 10001st prime
 * Problem 7
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 *
 * https://projecteuler.net/problem=7
 */

use primes::primes;
use run::run;

pub fn main() {
    run("007.0", || {
        assert_eq!(prime(1), 2);
        assert_eq!(prime(2), 3);
        assert_eq!(prime(3), 5);
        assert_eq!(prime(4), 7);
        assert_eq!(prime(5), 11);
        assert_eq!(prime(6), 13);
    });
    run("007.1: prime", || {
        assert_eq!(prime(10_001), 104_743);
    });
    run("007.2: primes.nth", || {
        assert_eq!(primes().nth(10_000), Some(104_743));
    });
}

pub fn prime(n: u16) -> u64 {
    assert!(n >= 1, "prime(n): n must be >= 1, got: {}", n);
    if n == 1 {
        return 2;
    }
    let mut primes: Vec<u64> = vec![2];
    let mut i = 3;
    for _ in 1..n {
        while primes.iter().any(|p| i % p == 0) {
            i += 2;
        }
        primes.push(i);
    }
    i
}
