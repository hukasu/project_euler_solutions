use project_euler::primes_up_to;

fn main() {
    println!("{}", primes_up_to(2_000_000).into_iter().sum::<u64>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes_up_to_test() {
        assert_eq!(primes_up_to(10).into_iter().sum::<u64>(), 17);
        assert_eq!(primes_up_to(11).into_iter().sum::<u64>(), 28);
        assert_eq!(primes_up_to(15).into_iter().sum::<u64>(), 41);
    }
}
