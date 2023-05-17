fn main() {
    let r = (0_u64..100000000)
        .filter(|n| {
            n.eq(&n
                .to_string()
                .chars()
                .map(|d| (d.to_digit(10).unwrap() as u64).pow(5))
                .sum::<u64>())
        })
        .sum::<u64>();
    println!("{:?}", r);
}
