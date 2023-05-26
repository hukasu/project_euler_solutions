use project_euler::sequencial_distinct_prime_factors;

fn main() {
    let r = sequencial_distinct_prime_factors(4, 4, 200000);
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sequencial_distinct_prime_factors_test() {
        assert_eq!(sequencial_distinct_prime_factors(2, 2, 1000), Some(14));
        assert_eq!(sequencial_distinct_prime_factors(3, 3, 1000), Some(644));
    }
}
