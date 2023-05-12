use project_euler::collatz_sequence;

fn main() {
    let r: Vec<_> = (0..=1_000_000).map(|v| collatz_sequence(v).len()).collect();
    let max = *r.iter().max().unwrap();
    println!("{}", r.into_iter().position(|len| len == max).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collatz_sequence_test() {
        assert_eq!(
            collatz_sequence(13),
            vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
    }
}
