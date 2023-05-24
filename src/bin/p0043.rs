use project_euler::{is_prime_factor, permutations_of_digits_up_to_n, primes_up_to};

fn main() {
    let primes = primes_up_to(17);
    let r = permutations_of_digits_up_to_n(9)
        .into_iter()
        .filter(|s| !s.starts_with('0'))
        .filter(|s| {
            (1..8)
                .map(|i| &s[i..(i + 3)])
                .collect::<Vec<_>>()
                .into_iter()
                .zip(primes.iter())
                .all(|(slc, prm)| is_prime_factor(slc.parse::<u64>().unwrap(), *prm))
        })
        .map(|s| str::parse::<u64>(&s).unwrap())
        .sum::<u64>();
    println!("{:?}", r);
}
