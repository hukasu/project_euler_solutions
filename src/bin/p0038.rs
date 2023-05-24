use project_euler::pandigital_numbers;

fn main() {
    let r = (0..50_000)
        .filter_map(|i| {
            let products = (1..=9).map(|d| i * d).collect::<Vec<_>>();
            let slices = (1..=9).map(|i| &products[..i]).collect::<Vec<_>>();
            slices
                .iter()
                .position(|slc| pandigital_numbers(slc))
                .map(|pan| {
                    (
                        i,
                        slices[pan]
                            .iter()
                            .map(u64::to_string)
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    )
                })
        })
        .max_by(|(_, l), (_, r)| l.cmp(r));
    println!("{:?}", r);
}
