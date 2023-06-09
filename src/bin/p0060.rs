use std::collections::{BTreeMap, BTreeSet};

use project_euler::{is_pair_concatenation_prime, primes_up_to};

fn main() {
    let primes = primes_up_to(9999);
    let pairs = primes
        .iter()
        .map(|a| {
            (
                a,
                primes
                    .iter()
                    .filter(|b| b > &a)
                    .filter(|b| is_pair_concatenation_prime(a, b))
                    .collect::<BTreeSet<_>>(),
            )
        })
        .filter(|(_, l)| !l.is_empty())
        .collect::<BTreeMap<_, _>>();
    let trios = pairs
        .iter()
        .flat_map(|(a, inpairs)| {
            inpairs.iter().filter(|b| pairs.contains_key(*b)).map(|b| {
                (
                    (*a, *b),
                    inpairs
                        .intersection(&pairs[b])
                        .filter(|c| c > &b)
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                )
            })
        })
        .filter(|(_, l)| !l.is_empty())
        .collect::<BTreeMap<_, _>>();
    let quads = trios
        .iter()
        .flat_map(|((a, b), inpairs)| {
            inpairs.iter().filter(|c| pairs.contains_key(*c)).map(|c| {
                (
                    (*a, *b, *c),
                    inpairs
                        .intersection(&pairs[c])
                        .filter(|d| d > &c)
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                )
            })
        })
        .filter(|(_, l)| !l.is_empty())
        .collect::<BTreeMap<_, _>>();
    let quints = quads
        .iter()
        .flat_map(|((a, b, c), inpairs)| {
            inpairs.iter().filter(|d| pairs.contains_key(*d)).map(|d| {
                (
                    (*a, *b, *c, *d),
                    inpairs
                        .intersection(&pairs[d])
                        .filter(|e| e > &d)
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                )
            })
        })
        .filter(|(_, l)| !l.is_empty())
        .collect::<BTreeMap<_, _>>();
    let r = quints
        .into_iter()
        .flat_map(|((a, b, c, d), inpairs)| {
            inpairs
                .into_iter()
                .map(|e| a + b + c + d + e)
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_pair_concatenation_prime_test() {
        assert!(is_pair_concatenation_prime(&3, &7));
        assert!(is_pair_concatenation_prime(&7, &3));
        assert!(is_pair_concatenation_prime(&3, &109));
        assert!(is_pair_concatenation_prime(&3, &673));
        assert!(is_pair_concatenation_prime(&7, &109));
        assert!(is_pair_concatenation_prime(&7, &673));
        assert!(is_pair_concatenation_prime(&109, &673));
        assert!(!is_pair_concatenation_prime(&7, &11));
    }
}
