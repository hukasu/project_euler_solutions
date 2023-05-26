use project_euler::power_modulus;

fn main() {
    let r = (1..=1_000)
        .map(|n| power_modulus(n, n, 10_u64.pow(10)))
        .sum::<u64>();
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
    use project_euler::BigUInt;

    use super::*;

    #[test]
    fn power_modulus_test() {
        assert_eq!(
            (1..=10)
                .map(|n| power_modulus(n, n, 10_u64.pow(10)))
                .sum::<u64>(),
            405071317
        )
    }

    #[test]
    fn self_powers_test() {
        assert_eq!(
            (1..=10).map(|n| BigUInt::from(n).pow(n)).sum::<BigUInt>(),
            BigUInt::from(10405071317)
        );
    }
}
