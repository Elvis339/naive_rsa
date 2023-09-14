extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{One, Zero};
use rand::thread_rng;

#[derive(Debug)]
pub struct Prime {
    inner: BigUint,
}

impl Prime {
    pub fn new(bits: usize, rounds: usize) -> Self {
        loop {
            let candidate = Self::generate_random_biguint(bits);
            if Self::is_prime(&candidate, rounds) {
                return Self { inner: candidate };
            }
        }
    }

    pub fn get(&self) -> &BigUint {
        &self.inner
    }

    // Miller-Rabin primality test
    pub fn is_prime(n: &BigUint, rounds: usize) -> bool {
        let two = 2.to_biguint().unwrap();

        if *n < two {
            return false;
        }

        if *n == two {
            return true;
        }

        let mut d = n.clone() - BigUint::one();
        let mut r = 0;

        while &d % &two == BigUint::zero() {
            d /= 2.to_biguint().unwrap();
            r += 1;
        }

        let mut rng = rand::thread_rng();
        'outer: for _ in 0..rounds {
            let a = rng.gen_biguint_range(&two, n);
            let mut x = a.modpow(&d, n);

            if x == BigUint::one() || x == n - BigUint::one() {
                continue;
            }

            /// Probabilistic method we could add star witnesses to in addition to make sure the number is truly safe prime: https://www.youtube.com/watch?v=_MscGSN5J6o
            for _ in 0..r - 1 {
                x = x.modpow(&two, n);
                if x == n - BigUint::one() {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }

    fn generate_random_biguint(bits: usize) -> BigUint {
        let mut rng = thread_rng();
        rng.gen_biguint(bits as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::Prime;

    #[test]
    fn prime_generation() {
        let mut tests = 10;
        while tests > 0 {
            let prime = Prime::new(256, 5);
            assert_eq!(Prime::is_prime(prime.get(), 5), true);
            tests -= 1;
        }
    }
}
