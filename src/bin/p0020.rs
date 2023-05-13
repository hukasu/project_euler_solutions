use project_euler::{mul_digit_lists, string_to_list_of_digits};

fn main() {
    println!(
        "{}",
        (1..=100)
            .map(|n| string_to_list_of_digits(&n.to_string()))
            .reduce(|a, b| mul_digit_lists(&a, &b))
            .unwrap()
            .into_iter()
            .map(u64::from)
            .sum::<u64>()
    )
}
