/// Represents the mantissa of a fraction where 1 is the numerator.
#[derive(Debug, PartialEq, Clone)]
pub struct UnitFraction(Vec<u8>);

impl UnitFraction {
    /// Generates the unit fraction of `x` up to `precision` decimal places.
    pub fn new(x: u64, precision: usize) -> Self {
        if x == 0 {
            panic!("Zero division.")
        } else if x == 1 {
            return Self(vec![]);
        };

        let mut res = vec![];
        let mut numerator = 1;
        while numerator > 0 && res.len() < precision {
            numerator *= 10;
            res.push((numerator / x) as u8);
            numerator %= x;
        }
        Self(res)
    }

    /// Get the precision of the UnitFraction
    pub fn precision(&self) -> usize {
        self.0.len()
    }

    /// Check if unit fraction is cyclic.
    pub fn is_cyclic(&self) -> bool {
        self.get_cycle_length() > 0
    }

    pub fn get_cycle_length(&self) -> usize {
        for window_len in 1..self.precision() {
            for window_start in 0..self.precision() {
                if window_start + window_len > self.precision() {
                    break;
                } else if self.precision() - (window_start + window_len) < 2 * window_len {
                    continue;
                }
                let window = self.0[window_start..(window_start + window_len)]
                    .iter()
                    .cycle();
                let cmp_window = self.0[window_start..].iter();
                if window.zip(cmp_window).all(|(l, r)| l.eq(r)) {
                    return window_len;
                }
            }
        }

        0
    }
}

impl From<u64> for UnitFraction {
    /// Generates the unit fraction of `x` up to 64 decimal places.
    fn from(x: u64) -> Self {
        Self::new(x, 64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_fraction_test() {
        assert_eq!(UnitFraction::from(1), UnitFraction(vec![]));
        assert_eq!(UnitFraction::from(2), UnitFraction(vec![5]));
        assert_eq!(UnitFraction::from(3), UnitFraction(vec![3; 64]));
        assert_eq!(UnitFraction::from(4), UnitFraction(vec![2, 5]));
        assert_eq!(UnitFraction::from(5), UnitFraction(vec![2]));
        let mut u6 = vec![1];
        u6.extend(vec![6; 63]);
        assert_eq!(UnitFraction::from(6), UnitFraction(u6));
        let u7 = vec![1, 4, 2, 8, 5, 7]
            .into_iter()
            .cycle()
            .take(64)
            .collect();
        assert_eq!(UnitFraction::from(7), UnitFraction(u7));
        let u7 = vec![1, 4, 2, 8, 5, 7]
            .into_iter()
            .cycle()
            .take(10)
            .collect();
        assert_eq!(UnitFraction::new(7, 10), UnitFraction(u7));
    }

    #[test]
    fn is_cyclic_test() {
        assert!(!UnitFraction::from(1).is_cyclic());
        assert!(!UnitFraction::from(2).is_cyclic());
        assert!(UnitFraction::from(3).is_cyclic());
        assert!(!UnitFraction::from(4).is_cyclic());
        assert!(!UnitFraction::from(5).is_cyclic());
        assert!(UnitFraction::from(6).is_cyclic());
        assert!(UnitFraction::from(7).is_cyclic());
        assert!(!UnitFraction::from(8).is_cyclic());
        assert!(UnitFraction::from(9).is_cyclic());
    }

    #[test]
    fn get_cycle_length_test() {
        assert_eq!(UnitFraction::from(3).get_cycle_length(), 1);
        assert_eq!(UnitFraction::from(6).get_cycle_length(), 1);
        assert_eq!(UnitFraction::from(7).get_cycle_length(), 6);
        assert_eq!(UnitFraction::from(9).get_cycle_length(), 1);
    }
}
