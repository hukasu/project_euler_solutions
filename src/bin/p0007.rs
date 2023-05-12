use project_euler::nth_prime;

fn main() {
    println!("{}", nth_prime(10_001));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(4), 7);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(6), 13);
    }
}