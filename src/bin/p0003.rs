use project_euler::get_prime_factors;

fn main() {
    println!("{:?}", get_prime_factors(600851475143));
}

#[cfg(test)]
mod tests {
    use super::*;

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
