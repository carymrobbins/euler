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
    let res = divisible_by_all(1, 20);
    println!("Result: {}", res);
}

pub fn divisible_by_all(min: u64, max: u64) -> u64 {
    if max < min {
        panic!("max cannot be less than min: {} < {}", max, min);
    }
    if min < 1 {
        panic!("min cannot be less than 1: {} < 1", min);
    }
    let mut ms = vec![];
    for d in (min..(max+1)).rev() {
        let mut add = true;
        for m in ms.iter() {
            if m % d == 0 {
                add = false;
                break;
            }
        }
        if add {
            ms.push(d);
        }
    }
    let mut p = ms.iter().product();
    for m in min..(max+1) {
        if m == 1 {
            continue;
        }
        loop {
            let p2 = p / m;
            let mut ok = true;
            for n in min..(max+1) {
                if n == 1 {
                    continue;
                }
                if p2 % n != 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                p = p2;
            } else {
                break;
            }
        }
    }
    p
}
