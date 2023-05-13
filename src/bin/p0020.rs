use project_euler::{string_to_list_of_digits, mul_digit_lists};

fn main() {
    println!("{}", (1..=100).map(|n| string_to_list_of_digits(&n.to_string())).reduce(|a, b| mul_digit_lists(&a, &b)).unwrap().into_iter().map(u64::from).sum::<u64>())
}