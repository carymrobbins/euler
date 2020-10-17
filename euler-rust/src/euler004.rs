/*
 * Largest palindrome product
 * Problem 4
 *
 * A palindromic number reads the same both ways. The largest palindrome made
 * from the product of two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */

use run::run;

pub fn main() {
    run("004", || {
        assert_eq!(largest_palindrome_product(2), 9009);
        assert_eq!(largest_palindrome_product(3), 906609);
    });
}

fn largest_palindrome_product(n_digits: u32) -> u64 {
    let min = 10u64.pow(n_digits - 1);
    let max = 10u64.pow(n_digits) - 1;
    for z in ((min * min)..=(max * max)).rev() {
        if !is_palindrome(z) {
            continue;
        }
        for x in (min..=max).rev() {
            let r = z % x;
            if r != 0 {
                continue;
            }
            let q = z / x;
            if q / (max + 1) != 0 {
                continue;
            }
            return z;
        }
    }
    panic!("No palindrome found between {} and {}", min, max);
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let rs = s.chars().rev().collect::<String>();
    s == rs
}
