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
    let start = if min == 1 { 2 } else { min };
    let range = || start..=max;
    let mut multipliers = vec![];
    for d in range().rev() {
        if multipliers.iter().all(|m| m % d != 0) {
            multipliers.push(d);
        }
    }
    let mut result = multipliers.iter().product();
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
