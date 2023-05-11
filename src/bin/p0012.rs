use project_euler::first_triangle_number_with_over_n_factors;

fn main() {
    println!("{}", first_triangle_number_with_over_n_factors(500));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_triangle_number_with_over_n_factors_test() {
        assert_eq!(first_triangle_number_with_over_n_factors(5), 28);
        assert_eq!(first_triangle_number_with_over_n_factors(6), 28);
        assert_eq!(first_triangle_number_with_over_n_factors(7), 36);
    }
}
