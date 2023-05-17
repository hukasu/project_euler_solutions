use project_euler::spiral_diagonals_sum;

fn main() {
    println!("{:?}", spiral_diagonals_sum(1001));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral_diagonals_sum_test() {
        assert_eq!(spiral_diagonals_sum(5), 101);
    }
}
