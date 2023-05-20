use project_euler::{primes_up_to, truncable_prime};

fn main() {
    let primes = primes_up_to(1_000_000);
    println!(
        "{:?}",
        primes
            .into_iter()
            .filter(|x| truncable_prime(*x))
            .sum::<u64>()
    )
}
