use num_bigint::BigUint;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Galois {
    pub num: BigUint,
    pub prime: BigUint,
}

impl Galois {
    pub fn new(mut num: BigUint, prime: BigUint) -> Self {
        num %= &prime;
        Self {
            num: num,
            prime: prime,
        }
    }
}

impl std::ops::Add for Galois {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        // Validate the operation
        if self.prime != rhs.prime {
            panic!("You are adding elements of different primes.");
        }

        // 1. Add rhs directly into self's existing memory buffer
        self.num += rhs.num;

        // 2. Do the conditional check
        if self.num >= self.prime {
            self.num -= &self.prime;
        }

        // 3. Return the recycled struct! No new allocations!
        self
    }
}

impl std::ops::Sub for Galois {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        // Validate the operation
        if self.prime != rhs.prime {
            panic!("You are subtracting elements of different primes.");
        }

        if self.num >= rhs.num {
            self.num -= rhs.num;
        } else {
            self.num += &self.prime;
            self.num -= rhs.num;
        }

        self
    }
}

impl std::ops::Mul for Galois {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        // Validate the operation
        if self.prime != rhs.prime {
            panic!("You are multiplying elements of different primes.");
        }

        self.num *= rhs.num;

        if self.num >= self.prime {
            self.num %= &self.prime;
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------Part I--------------------
    // State and Boundary Initialization
    // Define the struct and the constructor
    // ----------------------------------------------

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

    // --------------------Part II-------------------
    // Equivalence and Formatting
    // Implementing PartialEq, Eq, and Debug.
    // ----------------------------------------------

    #[test] // Galois::new(0, 13) == Galois::new(0, 13)
    fn eq_check_zero() {
        assert_eq!(
            Galois::new(BigUint::from(0u64), BigUint::from(13u64)),
            Galois::new(BigUint::from(0u64), BigUint::from(13u64))
        );
    }

    #[test] // Galois::new(3, 17) == Galois::new(3, 17)
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

    // -------------------Part III-------------------
    // Linear Arithmetic
    // Overloading std::ops::Add and std::ops::Sub.
    // ----------------------------------------------

    #[test]
    #[should_panic] // We don't want add 2 elements with different primes
    #[allow(non_snake_case)]
    fn add_check_prime() {
        let one_in_F47 = Galois::new(BigUint::from(1u64), BigUint::from(47u64));
        let two_in_F53 = Galois::new(BigUint::from(2u64), BigUint::from(53u64));
        let _result = one_in_F47 + two_in_F53;
    }

    #[test] // Galois::new(big_num, big_prime) + Galois::new(0, big_prime) == Galois::new(big_num, big_prime)
    #[allow(non_snake_case)]
    fn add_check_zero() {
        let big_num = Galois::new(
            "553059516537161321408265876839".parse::<BigUint>().unwrap(),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let big_zero = Galois::new(
            BigUint::ZERO,
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let result = big_num.clone() + big_zero;
        assert_eq!(result, big_num);
    }

    #[test] // Galois::new(1, 59) + Galois::new(2, 59) == Galois::new(3, 59)
    #[allow(non_snake_case)]
    fn add_check_basic() {
        let one_in_F59 = Galois::new(BigUint::from(1u64), BigUint::from(59u64));
        let two_in_F59 = Galois::new(BigUint::from(2u64), BigUint::from(59u64));
        let result = one_in_F59 + two_in_F59;
        assert_eq!(
            result,
            Galois::new(BigUint::from(3u64), BigUint::from(59u64))
        );
    }

    #[test] // Galois::new(60, 61) + Galois::new(2, 61) == Galois::new(1, 61)
    #[allow(non_snake_case)]
    fn add_check_overload() {
        let sixty_in_F61 = Galois::new(BigUint::from(60u64), BigUint::from(61u64));
        let two_in_F61 = Galois::new(BigUint::from(2u64), BigUint::from(61u64));
        let result = sixty_in_F61 + two_in_F61;
        assert_eq!(
            result,
            Galois::new(BigUint::from(1u64), BigUint::from(61u64))
        );
    }

    #[test]
    #[should_panic] // We don't want subtract 2 elements with different primes
    #[allow(non_snake_case)]
    fn sub_check_prime() {
        let one_in_F67 = Galois::new(BigUint::from(1u64), BigUint::from(67u64));
        let two_in_F71 = Galois::new(BigUint::from(2u64), BigUint::from(71u64));
        let _result = one_in_F67 - two_in_F71;
    }

    #[test] // Galois::new(big_num, big_prime) - Galois::new(0, big_prime) == Galois::new(big_num, big_prime)
    #[allow(non_snake_case)]
    fn sub_check_zero() {
        let big_num = Galois::new(
            "553059516537161321408265876838".parse::<BigUint>().unwrap(),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let big_zero = Galois::new(
            BigUint::ZERO,
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let result = big_num.clone() - big_zero;
        assert_eq!(result, big_num);
    }

    #[test] // Galois::new(70, 73) - Galois::new(2, 73) == Galois::new(68, 73)
    #[allow(non_snake_case)]
    fn sub_check_basic() {
        let seventy_in_F73 = Galois::new(BigUint::from(70u64), BigUint::from(73u64));
        let two_in_F73 = Galois::new(BigUint::from(2u64), BigUint::from(73u64));
        let result = seventy_in_F73 - two_in_F73;
        assert_eq!(
            result,
            Galois::new(BigUint::from(68u64), BigUint::from(73u64))
        );
    }

    #[test] // Galois::new(1, 79) - Galois::new(2, 79) == Galois::new(78, 79)
    #[allow(non_snake_case)]
    fn sub_check_overload() {
        let one_in_F79 = Galois::new(BigUint::from(1u64), BigUint::from(79u64));
        let two_in_F79 = Galois::new(BigUint::from(2u64), BigUint::from(79u64));
        let result = one_in_F79 - two_in_F79;
        assert_eq!(
            result,
            Galois::new(BigUint::from(78u64), BigUint::from(79u64))
        );
    }

    // -------------------Part IV--------------------
    // Non-Linear Arithmetic
    // Overloading std::ops::Mul
    // ----------------------------------------------

    #[test]
    #[should_panic] // We don't want multiple 2 elements with different primes
    #[allow(non_snake_case)]
    fn mul_check_prime() {
        let one_in_F83 = Galois::new(BigUint::from(1u64), BigUint::from(83u64));
        let two_in_F89 = Galois::new(BigUint::from(2u64), BigUint::from(89u64));
        let _result = one_in_F83 * two_in_F89;
    }

    #[test] // Galois::new(big_num, big_prime) * Galois::new(0, big_prime) == Galois::new(0, big_prime)
    #[allow(non_snake_case)]
    fn mul_check_zero() {
        let big_num = Galois::new(
            "553059516537161321408265876837".parse::<BigUint>().unwrap(),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let big_zero = Galois::new(
            BigUint::ZERO,
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let result = big_num * big_zero.clone();
        assert_eq!(result, big_zero);
    }

    #[test] // Galois::new(big_num, big_prime) + Galois::new(0, big_prime) == Galois::new(big_num, big_prime)
    #[allow(non_snake_case)]
    fn mul_check_unit() {
        let big_num = Galois::new(
            "553059516537161321408265876836".parse::<BigUint>().unwrap(),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let big_one = Galois::new(
            BigUint::from(1u64),
            "553059516537161321408265876841".parse::<BigUint>().unwrap(),
        );
        let result = big_num.clone() * big_one;
        assert_eq!(result, big_num);
    }

    #[test] // Galois::new(3, 97) * Galois::new(2, 97) == Galois::new(6, 97)
    #[allow(non_snake_case)]
    fn mul_check_basic() {
        let three_in_F97 = Galois::new(BigUint::from(3u64), BigUint::from(97u64));
        let two_in_F97 = Galois::new(BigUint::from(2u64), BigUint::from(97u64));
        let result = three_in_F97 * two_in_F97;
        assert_eq!(
            result,
            Galois::new(BigUint::from(6u64), BigUint::from(97u64))
        );
    }

    #[test] // Galois::new(60, 101) * Galois::new(2, 101) == Galois::new(19, 101)
    #[allow(non_snake_case)]
    fn mul_check_overload() {
        let sixty_in_F101 = Galois::new(BigUint::from(60u64), BigUint::from(101u64));
        let two_in_F101 = Galois::new(BigUint::from(2u64), BigUint::from(101u64));
        let result = sixty_in_F101 * two_in_F101;
        assert_eq!(
            result,
            Galois::new(BigUint::from(19u64), BigUint::from(101u64))
        );
    }
}
