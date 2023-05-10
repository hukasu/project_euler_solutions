/// Checks if number is multiple of either `3` or `5`.
pub fn multiple_of_3_or_5(x: &u64) -> bool {
    (x % 3 == 0) || (x % 5 == 0)
}

/// Checks if `f` is a factor of `x`.
pub fn is_factor(x: u64, f: u64) -> bool {
    x % f == 0
}

/// Checks if `f` is a prime factor of `x`.
pub fn is_prime_factor(x: u64, f: u64) -> bool {
    is_factor(x, f) && get_prime_factors(f).is_empty()
}

/// Returns the factors of a number, excluding 1 and itself.
pub fn get_factors(x: u64) -> Vec<u64> {
    if x > 2 {
        (2..=x.div_euclid(2))
            .filter(|f| is_factor(x, *f))
            .collect()
    } else {
        vec![]
    }
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

/// Checks if number is palindrome.
pub fn is_palindrome(x: u64) -> bool {
    let s = x.to_string();
    let s2: String = s.chars().rev().collect();
    s == s2
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn prime_factor_test() {
        assert_eq!(get_prime_factors(2), vec![]);
        assert_eq!(get_prime_factors(3), vec![]);
        assert_eq!(get_prime_factors(4), vec![2]);
        assert_eq!(get_prime_factors(5), vec![]);
        assert_eq!(get_prime_factors(6), vec![2, 3]);
        assert_eq!(get_prime_factors(13195), vec![5, 7, 13, 29]);
    }

    #[test]
    fn factor_test() {
        assert_eq!(get_factors(2), vec![]);
        assert_eq!(get_factors(3), vec![]);
        assert_eq!(get_factors(4), vec![2]);
        assert_eq!(get_factors(5), vec![]);
        assert_eq!(get_factors(6), vec![2, 3]);
        assert_eq!(get_factors(9), vec![3]);
        assert_eq!(get_factors(12), vec![2, 3, 4, 6]);
    }

    #[test]
    fn palindrome_test() {
        assert!(is_palindrome(9));
        assert!(is_palindrome(99));
        assert!(is_palindrome(909));
        assert!(is_palindrome(9009));
        assert!(is_palindrome(90109));
    }
}