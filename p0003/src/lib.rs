/// Checks if `f` is a prime factor of `x`.
pub fn is_prime_factor(x: u64, f: u64) -> bool {
    x % f == 0 && get_prime_factors(f).len() == 0
}

/// Returns the prime factors of a number, excluding 1 and itself.
pub fn get_prime_factors(x: u64) -> Vec<u64> {
    if x > 2 {
        [2_u64]
            .into_iter()
            .chain((3..=(x as f64).sqrt().ceil() as u64).step_by(2))
            .filter(|f| is_prime_factor(x, *f))
            .collect()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::get_prime_factors;

    #[test]
    fn prime_factor_test() {
        assert_eq!(get_prime_factors(2), vec![]);
        assert_eq!(get_prime_factors(3), vec![]);
        assert_eq!(get_prime_factors(4), vec![2]);
        assert_eq!(get_prime_factors(5), vec![]);
        assert_eq!(get_prime_factors(6), vec![2, 3]);
        assert_eq!(get_prime_factors(13195), vec![5, 7, 13, 29]);
    }
}