use project_euler::is_amicable_number;

fn main() {
    println!(
        "{}",
        (0..10_000).filter(|n| is_amicable_number(*n)).sum::<u64>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_amicable_number_test() {
        assert!(is_amicable_number(220));
        assert!(is_amicable_number(284));
    }
}
