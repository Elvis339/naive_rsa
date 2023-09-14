use num_bigint::{BigInt, BigUint, Sign};
use num_traits::One;
use num_traits::Zero;

pub fn mod_inv(a: &BigUint, module: &BigUint) -> Option<BigUint> {
    let zero = BigInt::zero();
    let one = BigInt::one();

    // Convert BigUint inputs to BigInt and initialize variables for the algorithm
    let mut mn = (
        BigInt::from_biguint(Sign::Plus, module.clone()),
        BigInt::from_biguint(Sign::Plus, a.clone()),
    );
    let mut xy = (zero.clone(), one.clone());

    // Perform the extended Euclidean algorithm to find the modular inverse
    while mn.1 != zero {
        xy = (xy.1.clone(), xy.0.clone() - (&mn.0 / &mn.1) * xy.1.clone());
        mn = (mn.1.clone(), mn.0 % mn.1.clone());
    }

    // If the GCD is greater than one, a modular inverse does not exist
    if mn.0 > one {
        return None;
    }

    // Calculate the modular inverse and convert it back to BigUint
    let res = (xy.0 + BigInt::from(module.clone())) % BigInt::from(module.clone());
    Some(res.to_biguint().unwrap())
}
