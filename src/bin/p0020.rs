use project_euler::BigUInt;

fn main() {
    println!(
        "{}",
        (1..=100)
            .map(BigUInt::from)
            .reduce(|a, b| a * b)
            .unwrap()
            .sum_of_digits()
    )
}
