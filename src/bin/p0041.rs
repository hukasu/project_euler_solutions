use project_euler::{pandigital_numbers, primes_up_to};

fn main() {
    let r = primes_up_to(10_000_000)
        .into_iter()
        .filter(|d| pandigital_numbers(&[*d; 1]))
        .max();
    println!("{:?}", r);
}
