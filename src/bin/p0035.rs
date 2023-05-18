use project_euler::{circular_shifts_of_a_number, is_prime, primes_up_to};

fn main() {
    let primes = primes_up_to(1_000_000)
        .into_iter()
        .filter(|p| {
            let circular = circular_shifts_of_a_number(*p);
            circular.into_iter().all(is_prime)
        })
        .count();

    println!("{:?}", primes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circular_shifts_of_a_number_test() {
        assert_eq!(circular_shifts_of_a_number(197), vec![197, 971, 719]);
    }
}
