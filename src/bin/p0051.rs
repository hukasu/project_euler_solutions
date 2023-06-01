use std::collections::BTreeSet;

use project_euler::{primes_up_to, replace_digits};

fn main() {
    const DIGITS: u32 = 6;
    let mut masks = (0..DIGITS).fold(BTreeSet::from_iter(vec![1]), |mut set, _| {
        let end_zero = set.iter().map(|o| o * 10).collect::<Vec<_>>();
        let end_one = end_zero.iter().map(|z| z + 1).collect::<Vec<_>>();
        set.extend(end_zero);
        set.extend(end_one);
        set
    });
    // Remove full mask
    let max_mask = *masks.iter().max().unwrap();
    masks.remove(&max_mask);

    let primes = BTreeSet::from_iter(primes_up_to(10_u64.pow(DIGITS)));

    let r = primes
        .iter()
        .map(|p| {
            let max = masks
                .iter()
                .filter(|msk| (msk as &&u64).ilog10() <= p.ilog10())
                .filter_map(|m| {
                    let family = (0..=9)
                        .filter_map(|digit| {
                            if digit == 0 && (m as &u64).ilog10() == p.ilog10() {
                                None
                            } else {
                                let nn = replace_digits(*p, *m, digit);
                                if primes.contains(&nn) {
                                    Some(nn)
                                } else {
                                    None
                                }
                            }
                        })
                        .collect::<Vec<_>>();
                    if family.contains(p) {
                        Some(family.len())
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            (p, max)
        })
        .find(|(_, family)| family >= &8);
    println!("{:?}", r);
}
