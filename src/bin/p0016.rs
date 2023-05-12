use project_euler::power_digit_sum;

fn main() {
    println!("{}", power_digit_sum(2, 1000));
}

#[cfg(test)]
mod test {
    use super::*;
    use project_euler::mul_digit_lists;

    #[test]
    fn mul_digit_lists_test() {
        assert_eq!(mul_digit_lists(&vec![2], &vec![2]), vec![4]);
        assert_eq!(mul_digit_lists(&vec![2], &vec![8]), vec![1, 6]);
        assert_eq!(mul_digit_lists(&vec![5], &vec![5]), vec![2, 5]);
        assert_eq!(mul_digit_lists(&vec![5, 0], &vec![2]), vec![1, 0, 0]);
    }

    #[test]
    fn power_sum_test() {
        assert_eq!(power_digit_sum(2, 15), 26);
    }
}
