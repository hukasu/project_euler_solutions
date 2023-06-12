use std::collections::BTreeMap;

use project_euler::number_to_sorted_list_of_digits;

fn main() {
    let r = (1_u64..10000)
        .map(|n| n.pow(3))
        .map(|n| {
            (
                n,
                number_to_sorted_list_of_digits(&n)
                    .into_iter()
                    .collect::<String>(),
            )
        })
        .fold(BTreeMap::default(), |mut map, (cur, digits)| {
            map.entry(digits)
                .and_modify(|v: &mut Vec<u64>| v.push(cur))
                .or_insert(vec![cur]);
            map
        })
        .into_iter()
        .filter(|entry| entry.1.len() == 5)
        .fold(u64::MAX, |min, entry| {
            *entry.1.iter().min().unwrap_or(&u64::MAX).min(&min)
        });
    println!("{}", r);
}
