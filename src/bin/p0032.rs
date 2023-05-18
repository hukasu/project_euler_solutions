use std::collections::BTreeMap;

const ONE_THROUGH_NINE: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let r = (1..=100)
        .flat_map(|a| {
            ((a + 1)..=10000)
                .map(|b| (a * b, (a, b)))
                .collect::<Vec<_>>()
        })
        .filter(|(p, (mp, mr))| {
            let mut s = p.to_string();
            s.push_str(&mp.to_string());
            s.push_str(&mr.to_string());
            let mut d = s.chars().collect::<Vec<_>>();
            d.sort();
            d.eq(&ONE_THROUGH_NINE)
        })
        .collect::<BTreeMap<u64, (u64, u64)>>()
        .keys()
        .sum::<u64>();
    println!("{:?}", r);
}
