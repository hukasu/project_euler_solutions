use std::collections::BTreeSet;

use project_euler::{is_prime, sum_of_consecutive_primes};

fn main() {
    let set = (0..=10)
        .flat_map(|skp| {
            sum_of_consecutive_primes(5000, skp)
                .into_iter()
                .enumerate()
                .filter(|(_, p)| is_prime(*p))
        })
        .filter(|(_, p)| p < &1000000)
        .collect::<BTreeSet<_>>();
    let r = set.iter().max_by(|(l, _), (r, _)| l.cmp(r));
    println!("{:?}", r);
}
