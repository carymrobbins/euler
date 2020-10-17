/*
 * Largest prime factor
 * Problem 3
 *
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143?
 */

use primes::primes;
use run::run;

pub fn main() {
    run("003", || {
        assert_eq!(largest_prime_factor(13_195), 29);
        assert_eq!(largest_prime_factor(600_851_475_143), 6_857);
    });
}

fn largest_prime_factor(mut n: u64) -> u64 {
    assert!(n > 0, "largest_prime_factor(n) n must be > 0");
    // If we don't find any prime factor, then we are prime and thus are
    // our own largest prime factor.
    let mut res: u64 = n;
    for p in primes() {
        if n == 1 {
            break;
        }
        if n % p == 0 {
            res = p;
            // Reduce n to decrease search space.
            n /= p;
        }
    }
    res
}
