use std::collections::BTreeSet;

fn main() {
    let indexes = BTreeSet::from([1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]);
    let r = (0..100_000)
        .flat_map(|d| d.to_string().chars().collect::<Vec<_>>())
        .enumerate()
        .filter(|(i, _)| indexes.contains(i))
        .map(|(_, c)| c.to_digit(10).unwrap() as u64)
        .product::<u64>();
    println!("{:?}", r);
}
