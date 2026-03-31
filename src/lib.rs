use num_bigint::BigUint;

#[derive(PartialEq, Eq, Debug, Clone)]
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

    #[test] // Galois::new(0, 3) = 0
    fn construction_check_zero() {
        let result = Galois::new(BigUint::from(0u64), BigUint::from(3u64));
        assert_eq!(result.num, BigUint::ZERO);
    }

    #[test] // Galois::new(3, 5) = 3
    fn construction_check_basic() {
        let result = Galois::new(BigUint::from(3u64), BigUint::from(5u64));
        assert_eq!(result.num, BigUint::from(3u64));
    }

    #[test] // Galois::new(9, 7) = 2
    fn construction_check_modulus() {
        let result = Galois::new(BigUint::from(9u64), BigUint::from(7u64));
        assert_eq!(result.num, BigUint::from(2u64));
    }

    #[test] // Galois::new(22，11) = 0
    fn construction_check_multiple() {
        let result = Galois::new(BigUint::from(22u64), BigUint::from(11u64));
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

    #[test] // Galois::new(0, 13) == Galois::new(0, 13)
    fn eq_check_zero() {
        assert_eq!(
            Galois::new(BigUint::from(0u64), BigUint::from(13u64)),
            Galois::new(BigUint::from(0u64), BigUint::from(13u64))
        );
    }

    #[test] // Galois::new(3, 17)  == Galois::new(3, 17)
    fn eq_check_basic() {
        assert_eq!(
            Galois::new(BigUint::from(3u64), BigUint::from(17u64)),
            Galois::new(BigUint::from(3u64), BigUint::from(17u64))
        );
    }

    #[test] // Galois::new(20, 19) == Galois::new(1, 19)
    fn eq_check_modulus() {
        assert_eq!(
            Galois::new(BigUint::from(20u64), BigUint::from(19u64)),
            Galois::new(BigUint::from(1u64), BigUint::from(19u64))
        );
    }

    #[test] // Galois::new(46, 23) == Galois::new(23, 23)
    fn eq_check_multiple() {
        assert_eq!(
            Galois::new(BigUint::from(46u64), BigUint::from(23u64)),
            Galois::new(BigUint::from(23u64), BigUint::from(23u64))
        );
    }

    #[test] //
    fn eq_check_big_prime() {
        assert_eq!(
            Galois::new(
                "553059516537161321408265876840".parse::<BigUint>().unwrap(),
                "553059516537161321408265876841".parse::<BigUint>().unwrap(),
            ),
            Galois::new(
                "553059516537161321408265876840".parse::<BigUint>().unwrap(),
                "553059516537161321408265876841".parse::<BigUint>().unwrap(),
            )
        );
    }

    #[test] // Galois::new(0, 29) != Galois::new(1, 29)
    fn eq_check_unequal_num() {
        assert_ne!(
            Galois::new(BigUint::from(0u64), BigUint::from(29u64)),
            Galois::new(BigUint::from(1u64), BigUint::from(29u64))
        );
    }

    #[test] // Galois::new(0, 31) != Galois::new(0, 37)
    fn eq_check_unequal_prime() {
        assert_ne!(
            Galois::new(BigUint::from(0u64), BigUint::from(31u64)),
            Galois::new(BigUint::from(0u64), BigUint::from(37u64))
        );
    }

    #[test] //
    #[allow(non_snake_case)]
    fn fmt_check() {
        let one_in_F37 = Galois::new(BigUint::from(1u64), BigUint::from(37u64));
        assert_eq!(format!("{one_in_F37:?}"), "Galois { num: 1, prime: 37 }");
    }

    #[test] // Galois::new(2, 41) == Galois::new(2, 41).clone()
    #[allow(non_snake_case)]
    fn clone_check() {
        let two_in_F41 = Galois::new(BigUint::from(2u64), BigUint::from(41u64));
        let two_in_F41_cloned = two_in_F41.clone();
        assert_eq!(two_in_F41, two_in_F41_cloned);
    }
}
