fn main() {
    let extract_digits = |n: u64| {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    };
    let mut factorials = (1..=9)
        .scan(1, |p, n| {
            let r = *p * n;
            *p = r;
            Some(r)
        })
        .collect::<Vec<_>>();
    factorials.insert(0, 1);
    let r = (0..10_u64.pow(7))
        .filter(|n| {
            n == &extract_digits(*n)
                .into_iter()
                .map(|c| factorials[c as usize])
                .sum::<u64>()
        })
        .skip(2)
        .sum::<u64>();
    println!("{:?}", r);
}
