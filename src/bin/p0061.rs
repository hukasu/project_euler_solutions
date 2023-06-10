use std::{collections::BTreeSet, time::Instant};

use project_euler::{
    cyclic_number, nth_heptagonal_number, nth_hexagonal_number, nth_octagonal_number,
    nth_pentagonal_number, nth_square_number, nth_triangle_number,
};

fn main() {
    let s = Instant::now();
    let triangle = (1..10000)
        .filter_map(|n| {
            let t = nth_triangle_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let square = (1..10000)
        .filter_map(|n| {
            let t = nth_square_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let pentagonal = (1..10000)
        .filter_map(|n| {
            let t = nth_pentagonal_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let hexagonal = (1..10000)
        .filter_map(|n| {
            let t = nth_hexagonal_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let heptagonal = (1..10000)
        .filter_map(|n| {
            let t = nth_heptagonal_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let octagonal = (1..10000)
        .filter_map(|n| {
            let t = nth_octagonal_number(n);
            if (1000..10000).contains(&t) {
                Some(t)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    let all = triangle
        .iter()
        .chain(square.iter())
        .chain(pentagonal.iter())
        .chain(hexagonal.iter())
        .chain(heptagonal.iter())
        .chain(octagonal.iter())
        .collect::<BTreeSet<_>>();
    let r = all
        .iter()
        .map(|n| [*n])
        .flat_map(|[l]| {
            all.iter()
                .filter(|r| cyclic_number(l, r, 2))
                .map(|r| [l, r])
                .collect::<BTreeSet<_>>()
        })
        .flat_map(|[n, l]| {
            all.iter()
                .filter(|r| cyclic_number(l, r, 2))
                .map(|r| [n, l, r])
                .collect::<BTreeSet<_>>()
        })
        .flat_map(|[n1, n2, l]| {
            all.iter()
                .filter(|r| cyclic_number(l, r, 2))
                .map(|r| [n1, n2, l, r])
                .collect::<BTreeSet<_>>()
        })
        .flat_map(|[n1, n2, n3, l]| {
            all.iter()
                .filter(|r| cyclic_number(l, r, 2))
                .map(|r| [n1, n2, n3, l, r])
                .collect::<BTreeSet<_>>()
        })
        .flat_map(|[n1, n2, n3, n4, l]| {
            all.iter()
                .filter(|r| cyclic_number(l, r, 2))
                .map(|r| [n1, n2, n3, n4, l, r])
                .collect::<BTreeSet<_>>()
        })
        .filter(|[a, _, _, _, _, b]| cyclic_number(b, a, 2))
        .filter(|l| BTreeSet::from_iter(l).len() == 6)
        .filter(|l| {
            let mut masks = l
                .iter()
                .map(|n| {
                    (if triangle.contains(n) { 0b1_u64 } else { 0b0 }
                        | if square.contains(n) { 0b10 } else { 0b0 }
                        | if pentagonal.contains(n) { 0b100 } else { 0b0 }
                        | if hexagonal.contains(n) { 0b1000 } else { 0b0 }
                        | if heptagonal.contains(n) { 0b10000 } else { 0b0 }
                        | if octagonal.contains(n) { 0b100000 } else { 0b0 })
                })
                .collect::<Vec<_>>();
            let mut r = true;
            while !masks.is_empty() {
                let powtwoopt = masks.iter().position(|n| n.is_power_of_two());
                if let Some(powtwo) = powtwoopt {
                    let powtwo = masks.remove(powtwo);
                    for m in &mut masks {
                        *m &= !powtwo;
                    }
                } else {
                    r = false;
                    break;
                }
            }
            r
        })
        .map(|l| l.iter().map(|n| **n).sum::<u64>())
        .collect::<BTreeSet<_>>();
    println!("{:?}", r);
    println!("{:?}", s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cyclic_number_test() {
        assert!(cyclic_number(&8128, &2882, 2));
        assert!(cyclic_number(&2882, &8281, 2));
        assert!(cyclic_number(&8281, &8128, 2));
        assert!(cyclic_number(&820000081, &8128, 2));
        assert!(cyclic_number(&8281, &810000028, 2));
        assert!(!cyclic_number(&5192, &9633, 2));
    }
}
