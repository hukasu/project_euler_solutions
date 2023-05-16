use project_euler::QuadraticFormula;

fn main() {
    let r = (-1000..1000)
        .flat_map(|a| {
            (0..=1000)
                .map(|b| QuadraticFormula::new(a, b))
                .collect::<Vec<_>>()
        })
        .map(|qf| {
            let seq = qf.longest_prime_sequence();
            (qf, seq)
        })
        .max_by(|(_, l), (_, r)| l.cmp(r))
        .unwrap();
    println!("{:?}", r.0);
}
