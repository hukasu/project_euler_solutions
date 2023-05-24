use std::collections::BTreeMap;

use project_euler::pandigital_numbers;

fn main() {
    let r = (1..=100)
        .flat_map(|a| ((a + 1)..=10000).map(|b| [a * b, a, b]).collect::<Vec<_>>())
        .filter(|ns| ns.iter().map(u64::to_string).collect::<String>().len() == 9)
        .filter(|ns| pandigital_numbers(ns, false))
        .map(|[p, mp, ms]| (p, (mp, ms)))
        .collect::<BTreeMap<u64, (u64, u64)>>()
        .keys()
        .sum::<u64>();
    println!("{:?}", r);
}
