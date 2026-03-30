use num_bigint::BigUint;

pub struct Galois {
    pub num: BigUint,
    pub prime: BigUint,
}

impl Galois {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        Self {
            num: num % &prime,
            prime: prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Galois::new(3, 5) = 3
    fn construction_check_basic() {
        let result = Galois::new(BigUint::from(3u64), BigUint::from(5u64));
        assert_eq!(result.num, BigUint::from(3u64));
    }

    #[test] // Galois::new(0, 7) = 0
    fn construction_check_zero() {
        let result = Galois::new(BigUint::from(0u64), BigUint::from(7u64));
        assert_eq!(result.num, BigUint::ZERO);
    }

    #[test] // Galois::new(13, 11) = 2
    fn construction_check_modulus() {
        let result = Galois::new(BigUint::from(13u64), BigUint::from(11u64));
        assert_eq!(result.num, BigUint::from(2u64));
    }

    #[test] // Galois::new(26, 13) = 0
    fn construction_check_multiple() {
        let result = Galois::new(BigUint::from(26u64), BigUint::from(13u64));
        assert_eq!(result.num, BigUint::ZERO);
    }

    #[test] // 553059516537161321408265876841 is a verified prime.
    fn construction_check_big_prime() {
        let result = Galois::new(
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        assert_eq!(result.num, BigUint::ZERO);
    }

    #[test]
    #[should_panic] // We don't want the prime to be 0.
    fn construction_zero_prime() {
        let _result = Galois::new(BigUint::from(17u64), BigUint::ZERO);
    }
}
