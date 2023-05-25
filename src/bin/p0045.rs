use std::collections::BTreeSet;

use project_euler::{hexagonal_number, nth_triangle_numbers, pentagon_number};

fn main() {
    let triangle = (1..100_000)
        .map(nth_triangle_numbers)
        .collect::<BTreeSet<_>>();
    let pentagonal = (1..100_000).map(pentagon_number).collect::<BTreeSet<_>>();
    let hexagonal = (1..100_000).map(hexagonal_number).collect::<BTreeSet<_>>();
    let tp = triangle
        .intersection(&pentagonal)
        .cloned()
        .collect::<BTreeSet<_>>();
    println!("{:?}", tp.intersection(&hexagonal).collect::<Vec<_>>());
}
