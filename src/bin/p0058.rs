use project_euler::{is_prime, vertices_of_number_spiral};

fn main() {
    let vertices = vertices_of_number_spiral(99999)
        .unwrap()
        .collect::<Vec<_>>();
    let r = vertices
        .as_slice()
        .chunks(4)
        .scan((0_u64, 1_u64), |(primes, count), wind| {
            *primes += wind.iter().take(3).filter(|n| is_prime(**n)).count() as u64;
            *count += 4;
            Some(*primes as f64 / *count as f64)
        })
        .position(|perc| perc < 0.10)
        .map(|len| 2_u64 * (len as u64 + 1) + 1);
    println!("{:?}", r);
}
