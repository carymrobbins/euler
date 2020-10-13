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

pub fn main() {
    assert_eq!(divisible_by_all(1, 10), 2520);
    assert_eq!(divisible_by_all(1, 20), 232792560);
    println!("OK");
}

pub fn divisible_by_all(min: u64, max: u64) -> u64 {
    if max < min {
        panic!("max cannot be less than min: {} < {}", max, min);
    }
    if min < 1 {
        panic!("min cannot be less than 1: {} < 1", min);
    }
    if min == 1 && max == 1 {
        return 1;
    }
    // Ensure our range does not include 1 as it's an unnecessary check.
    let start = if min == 1 { 2 } else { min };
    // Create a closure for building our range.
    let range = || start..=max;
    let mut result: u64 = 1;
    for d in range().rev() {
        if result % d != 0 {
            result *= d;
        }
    }
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
