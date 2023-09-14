use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, ToPrimitive};

use crate::prime::Prime;
use crate::utils::mod_inv;

#[derive(Debug)]
pub struct RSA {
    /// Product of two random prime numbers `p` and `q`
    n: BigUint,

    /// Private key
    d: BigUint,

    e: BigUint,
}

impl RSA {
    pub fn new() -> Self {
        let p = Prime::new(256, 5);
        let q = Prime::new(256, 5);

        let p_prime = p.get().clone();
        let q_prime = q.get().clone();

        let n = &p_prime * &q_prime;
        let phi = (&p_prime - BigUint::one()) * (&q_prime - BigUint::one());
        let e = 0x10001.to_biguint().unwrap();
        let d = mod_inv(&e, &phi).expect("could not calculate private key");

        Self { n, d, e }
    }

    pub fn encrypt(&self, msg: String) -> Vec<BigUint> {
        let msg_bytes = msg.as_bytes();
        msg_bytes
            .iter()
            .map(|&byte| {
                let byte_biguint = byte.to_biguint().unwrap();
                byte_biguint.modpow(&self.e, &self.n)
            })
            .collect()
    }

    pub fn decrypt(&self, ciphertext: Vec<BigUint>) -> String {
        let decrypted_bytes: Vec<u8> = ciphertext
            .iter()
            .map(|c| c.modpow(&self.d, &self.n).to_u8().unwrap())
            .collect();
        String::from_utf8(decrypted_bytes).expect("Decryption failed")
    }
}

#[cfg(test)]
mod tests {
    use crate::rsa::RSA;

    #[test]
    fn encrypt_and_decrypt() {
        let rsa = RSA::new();
        let msg = "Hello world!".to_string();

        let decrypt = rsa.decrypt(rsa.encrypt(msg.clone()));
        assert_eq!(decrypt, msg);
    }
}
