use project_euler::is_prime;

fn main() {
    let squares = (1_u64..10_000).map(|n| n.pow(2)).collect::<Vec<_>>();
    let r = (3..1_000_000)
        .step_by(2)
        .filter(|n| !is_prime(*n))
        .find(|oddcomp| {
            !squares
                .iter()
                .filter(|sqr| oddcomp > &(*sqr * 2))
                .any(|sqr| is_prime(oddcomp - (*sqr * 2)))
        });
    println!("{:?}", r);
}
