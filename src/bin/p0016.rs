use project_euler::BigUInt;

fn main() {
    println!("{}", BigUInt::from(2).pow(1000).sum_of_digits());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_digit_lists_test() {
        assert_eq!(BigUInt::from(2) * BigUInt::from(2), BigUInt::from(4));
        assert_eq!(BigUInt::from(2) * BigUInt::from(8), BigUInt::from(16));
        assert_eq!(BigUInt::from(5) * BigUInt::from(5), BigUInt::from(25));
        assert_eq!(BigUInt::from(50) * BigUInt::from(2), BigUInt::from(100));
    }

    #[test]
    fn power_sum_test() {
        assert_eq!(BigUInt::from(2).pow(15).sum_of_digits(), 26);
    }
}
