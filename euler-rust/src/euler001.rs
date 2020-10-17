/* Multiples of 3 and 5
 * Problem 1
 *
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

use run::run;

pub fn main() {
    run("001", || {
        assert_eq!(sum_of_multiples_below(10, vec![3, 5]), 23);
        assert_eq!(sum_of_multiples_below(1000, vec![3, 5]), 233168);
    })
}

pub fn sum_of_multiples_below(x: u64, fs: Vec<u64>) -> u64 {
    let mut res: u64 = 0;
    for i in 2..x {
        for f in fs.iter() {
            if i % f == 0 {
                res += i;
                break;
            }
        }
    }
    res
}
