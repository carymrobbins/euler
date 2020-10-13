/*
 * Sum square difference
 * Problem 6
 *
 * The sum of the squares of the first ten natural numbers is,
 *
 * 1²+2²+...+10²=385
 *
 * The square of the sum of the first ten natural numbers is,
 *
 * (1+2+...+10)²=3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
 *
 * 3025 - 385 = 2640
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 *
 * https://projecteuler.net/problem=6
 */

use run::run;

#[allow(dead_code)]
pub fn main() {
    run("006", || {
        assert_eq!(sum_of_squares_diff_square_of_sum(10), 2640);
        assert_eq!(sum_of_squares_diff_square_of_sum(100), 25164150);
    });
}

pub fn sum_of_squares_diff_square_of_sum(max: u64) -> u64 {
    let sum_of_squares: u64 = (1..=max).map(|n| n.pow(2)).sum();
    let square_of_sum: u64 = (1..=max).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}
