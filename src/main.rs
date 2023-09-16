use num_traits::Zero;
use num_traits::One;
use num_integer::Integer;
use num_bigint::BigInt;

fn main() {
    let modulo: BigInt = 29.into(); // prime

    let mut i = BigInt::one();
    while i < modulo.clone() {
        let mut j = BigInt::one();
        while j < modulo.clone() {
            
            let result = mod_pow(i.clone(), j.clone(), modulo.clone());
            if result.is_one() {
                println!(
                    "Generator used: {} -> Order of the subgroup: {}",
                    i, j
                );
                break;
            }
            j = j + BigInt::one();
        }
        i = i + BigInt::one();
    }
}

fn mod_pow(base: BigInt, exp: BigInt, modulus: BigInt) -> BigInt {
    if modulus.is_one() {
        return BigInt::zero();
    }

    let mut result = BigInt::one();
    let mut base = base % &modulus;
    let mut exp = exp.clone();

    while exp > BigInt::zero() {
        if exp.is_odd() {
            result = (&result * &base) % &modulus;
        }

        exp = exp >> 1;
        base = (&base * &base) % &modulus;
    }

    result
}


