use std::sync::Mutex;

use lazy_static::lazy_static;

pub fn primes() -> Primes {
    Primes { index: 0 }
}

#[allow(dead_code)]
/// Mostly useful for debugging.
pub fn copy_prime_cache() -> Vec<u64> {
    PRIMES_CACHE.lock().unwrap().iter().map(|p| *p).collect()
}

lazy_static! {
    static ref PRIMES_CACHE: Mutex<Vec<u64>> = Mutex::new(vec![2, 3]);
}

pub struct Primes {
    index: usize,
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut cache = PRIMES_CACHE.lock().unwrap();
        let res: u64 = match cache.get(self.index) {
            Some(n) => *n,
            None => {
                let mut v: u64 = 3;
                for i in 2..=self.index {
                    while cache.iter().any(|p| v % p == 0) {
                        v += 2;
                    }
                    if cache.len() < i + 1 {
                        cache.push(v);
                    }
                }
                v
            }
        };
        self.index += 1;
        Some(res)
    }
}
