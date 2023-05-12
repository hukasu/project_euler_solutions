use project_euler::{square_of_sum, sum_of_squares};

fn main() {
    let r = square_of_sum(100) - sum_of_squares(100);
    println!("{r}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_of_squares_test() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn square_of_sum_test() {
        assert_eq!(square_of_sum(10), 3025);
    }
}