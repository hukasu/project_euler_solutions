use project_euler::smallest_multiple_of_all_through_x;

fn main() {
    let r = smallest_multiple_of_all_through_x(20);
    println!("{r:?}");
}

#[cfg(test)]
mod test {
    use project_euler::get_prime_factors_frequencies;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn multiple_of_all_test() {
        assert_eq!(smallest_multiple_of_all_through_x(10), 2520);
    }

    #[test]
    fn prime_factor_frequencies_test() {
        assert_eq!(get_prime_factors_frequencies(2), HashMap::from([(2, 1)]));
        assert_eq!(get_prime_factors_frequencies(3), HashMap::from([(3, 1)]));
        assert_eq!(get_prime_factors_frequencies(4), HashMap::from([(2, 2)]));
        assert_eq!(get_prime_factors_frequencies(5), HashMap::from([(5, 1)]));
        assert_eq!(
            get_prime_factors_frequencies(6),
            HashMap::from([(2, 1), (3, 1)])
        );
        assert_eq!(get_prime_factors_frequencies(16), HashMap::from([(2, 4)]));
        assert_eq!(
            get_prime_factors_frequencies(20),
            HashMap::from([(2, 2), (5, 1)])
        );
    }
}
