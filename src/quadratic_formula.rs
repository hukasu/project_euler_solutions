use crate::is_prime;

#[derive(Debug, PartialEq)]
pub struct QuadraticFormula(i64, i64);

impl QuadraticFormula {
    pub fn new(a: i64, b: i64) -> Self {
        Self(a, b)
    }

    pub fn solve(&self, x: i64) -> i64 {
        x.pow(2) + x * self.0 + self.1
    }

    pub fn longest_prime_sequence(&self) -> isize {
        (0..1)
            .cycle()
            .enumerate()
            .position(|(i, _)| {
                let s = self.solve(i as i64);
                if s > 0 {
                    !is_prime(s as u64)
                } else {
                    true
                }
            })
            .unwrap_or(0) as isize
            - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quadratic_formula_test() {
        assert_eq!(QuadraticFormula::new(1, 41).longest_prime_sequence(), 39);
        assert_eq!(
            QuadraticFormula::new(-79, 1601).longest_prime_sequence(),
            79
        );
    }
}
