/*
 * Smallest multiple
 * Problem 5
 *
 * 2520 is the smallest number that can be divided by each of the numbers from
 * 1 to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the
 * numbers from 1 to 20?
 *
 * https://projecteuler.net/problem=5
 */

use run::run;

#[allow(dead_code)]
pub fn main() {
    run("005", || {
        assert_eq!(divisible_by_all(1, 10), 2520);
        assert_eq!(divisible_by_all(1, 20), 232792560);
    });
}

pub fn divisible_by_all(min: u64, max: u64) -> u64 {
    assert!(max >= min, "max cannot be less than min: {} < {}", max, min);
    assert!(min >= 1, "min cannot be less than 1: {} < 1", min);
    if min == 1 && max == 1 {
        return 1;
    }
    // Ensure our range does not include 1 as it's an unnecessary check.
    let start = if min == 1 { 2 } else { min };
    // Create a closure for building our range.
    let range = || start..=max;
    // Start by building up our result as a product by iterating backwards
    // through our range. Start with the max and on each iteration see if our
    // current result is a multiple of d. If not, we need to create a new
    // product from it to ensure we have a number that is divisible by all
    // of the numbers from max to d.
    let mut result: u64 = 1;
    for d in range().rev() {
        if result % d != 0 {
            result *= d;
        }
    }
    // We now need to go back through our range and see if we can reduce
    // our result. We'll check for each d in the range and see if we can
    // divide our result by d and still have a number that is divisible by
    // all of the entries in our range. Continue dividing result by d while
    // this is true.
    for d in range() {
        loop {
            let q = result / d;
            if range().all(|n| q % n == 0) {
                result = q;
            } else {
                break;
            }
        }
    }
    result
}
