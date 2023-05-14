use project_euler::permutations_of_digits_up_to_n;

fn main() {
    // Million-th on a 1-index system
    println!(
        "{:?}",
        permutations_of_digits_up_to_n(9).into_iter().nth(999_999)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digit_permutation_test() {
        assert_eq!(
            permutations_of_digits_up_to_n(2)
                .into_iter()
                .collect::<Vec<_>>(),
            ["012", "021", "102", "120", "201", "210"]
        )
    }
}
