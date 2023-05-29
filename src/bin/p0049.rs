use std::collections::BTreeSet;

use project_euler::primes_up_to;

fn main() {
    let primes = primes_up_to(9999)
        .into_iter()
        .filter(|p| p > &1000)
        .collect::<Vec<_>>();
    let dists = primes
        .iter()
        .flat_map(|p1| {
            primes
                .iter()
                .filter(|p2| p2 > &p1)
                .flat_map(|p2| {
                    primes
                        .iter()
                        .filter(|p3| p3 > &p2)
                        .map(|p3| (*p1, *p2, *p3))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|(p1, p2, p3)| p1.abs_diff(*p2).eq(&p2.abs_diff(*p3)))
        .filter(|(p1, p2, p3)| {
            let p1set = BTreeSet::from_iter(p1.to_string().chars());
            let p2set = BTreeSet::from_iter(p2.to_string().chars());
            let p3set = BTreeSet::from_iter(p3.to_string().chars());
            p1set.eq(&p2set) && p1set.eq(&p3set)
        })
        .collect::<Vec<_>>();
    println!("{:?}", dists);
}
