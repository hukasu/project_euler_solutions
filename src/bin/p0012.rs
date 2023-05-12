use project_euler::first_triangle_number_with_over_n_factors;

fn main() {
    println!("{}", first_triangle_number_with_over_n_factors(500));
}

#[cfg(test)]
mod test {
    use super::*;
    use project_euler::get_factors;

    #[test]
    fn factor_test() {
        assert_eq!(get_factors(2), vec![]);
        assert_eq!(get_factors(3), vec![]);
        assert_eq!(get_factors(4), vec![2]);
        assert_eq!(get_factors(5), vec![]);
        assert_eq!(get_factors(6), vec![2, 3]);
        assert_eq!(get_factors(9), vec![3]);
        assert_eq!(get_factors(12), vec![2, 3, 4, 6]);
    }

    #[test]
    fn first_triangle_number_with_over_n_factors_test() {
        assert_eq!(first_triangle_number_with_over_n_factors(5), 28);
        assert_eq!(first_triangle_number_with_over_n_factors(6), 28);
        assert_eq!(first_triangle_number_with_over_n_factors(7), 36);
    }
}
