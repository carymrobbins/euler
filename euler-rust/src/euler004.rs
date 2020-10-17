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
    // Detecting palindromes with str is not only simpler but slightly faster!
    run("004.1.1: is_palindrome_str", || {
        assert_eq!(largest_product(is_palindrome_str, 2), 9009);
    });
    run("004.1.2: is_palindrome_str", || {
        assert_eq!(largest_product(is_palindrome_str, 3), 906609);
    });
    run("004.2.1: is_palindrome_u64", || {
        assert_eq!(largest_product(is_palindrome_u64, 2), 9009);
    });
    run("004.2.2: is_palindrome_u64", || {
        assert_eq!(largest_product(is_palindrome_u64, 3), 906609);
    });
    run("004.3: largest_palindrome_product_from_three_digit_multipliers", || {
        assert_eq!(largest_palindrome_product_from_three_digit_multipliers(), 906609);
    });
}

fn largest_palindrome_product_from_three_digit_multipliers() -> u64 {
    let single_digits: Vec<u64> = (0..=9).rev().collect();
    let triple_digits: Vec<u64> = (100..=999).rev().collect();
    for a in single_digits.iter() {
        for b in single_digits.iter() {
            for c in single_digits.iter() {
                let res =
                      (a * 100_000) + (b * 10_000) + (c * 1_000)
                    + (c * 100) + (b * 10) + a;
                for d in triple_digits.iter() {
                    let r = res % d;
                    if r != 0 {
                        continue;
                    }
                    let q = res / d;
                    if q >= 100 && q <= 999 {
                        return res;
                    }
                }
            }
        }
    }
    panic!("No palindrome found for three-digit multipliers!");
}

fn largest_product(predicate: fn(u64) -> bool, n_digits: u32) -> u64 {
    let min = 10u64.pow(n_digits - 1);
    let max = 10u64.pow(n_digits) - 1;
    for z in ((min * min)..=(max * max)).rev() {
        if !predicate(z) {
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

fn is_palindrome_str(n: u64) -> bool {
    let s = n.to_string();
    let rs = s.chars().rev().collect::<String>();
    s == rs
}

fn is_palindrome_u64(mut n: u64) -> bool {
    let n_digits = get_n_digits(n);
    for i in 0..(n_digits / 2) {
        let r = n % 10;
        let p = 10u64.pow(n_digits - (i * 2) - 1);
        let l = n / p;
        if r != l {
            return false;
        }
        if n > r * p {
            n -= r * p;
        }
        n /= 10;
    }
    true
}

fn get_n_digits(n: u64) -> u32 {
    ((n as f64).log10() as u32) + 1
}
