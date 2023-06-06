use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Mul},
};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BigUInt(Vec<u8>);

impl BigUInt {
    pub fn number_of_digits(&self) -> usize {
        self.0.len()
    }

    pub fn pow(self, n: u64) -> Self {
        (0..n).fold(BigUInt::from(1), |sum, _| self.clone() * sum)
    }

    pub fn sum_of_digits(&self) -> u64 {
        self.0.iter().cloned().map(u64::from).sum()
    }

    pub fn reverse(self) -> Self {
        let mut rev: Vec<_> = self.0.iter().cloned().rev().collect();
        while rev.first() == Some(&0) {
            rev.remove(0);
        }
        Self(rev)
    }

    pub fn is_palindrome(&self) -> bool {
        self.0.iter().rev().eq(self.0.iter())
    }
}

impl Default for BigUInt {
    fn default() -> Self {
        Self(vec![0])
    }
}

impl Debug for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{};{}]",
            self.0.iter().map(|d| d.to_string()).collect::<String>(),
            self.0.len()
        )
    }
}

impl From<&str> for BigUInt {
    fn from(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
    }
}

impl From<u64> for BigUInt {
    fn from(n: u64) -> Self {
        n.to_string().as_str().into()
    }
}

impl Add<&BigUInt> for BigUInt {
    type Output = Self;

    fn add(self, rhs: &BigUInt) -> Self::Output {
        let mut l = self.0.iter().rev();
        let mut r = rhs.0.iter().rev();

        let mut carry = 0;
        let mut out = vec![];
        loop {
            match (l.next(), r.next()) {
                (Some(lv), Some(rv)) => {
                    let add = lv + rv + carry;
                    carry = add / 10;
                    out.push(add % 10);
                }
                (Some(lv), None) => {
                    let add = lv + carry;
                    carry = add / 10;
                    out.push(add % 10);
                }
                (None, Some(rv)) => {
                    let add = rv + carry;
                    carry = add / 10;
                    out.push(add % 10);
                }
                (None, None) => {
                    if carry > 0 {
                        out.push(carry);
                    }
                    break;
                }
            }
        }
        Self(out.into_iter().rev().collect())
    }
}

impl Add<BigUInt> for BigUInt {
    type Output = Self;

    fn add(self, rhs: BigUInt) -> Self::Output {
        self + &rhs
    }
}

impl AddAssign<BigUInt> for BigUInt {
    fn add_assign(&mut self, rhs: BigUInt) {
        *self = rhs + (self as &Self);
    }
}

impl Mul<BigUInt> for BigUInt {
    type Output = Self;

    fn mul(self, rhs: BigUInt) -> Self::Output {
        let mult = rhs.0.iter().rev().scan(0, |tens, cur| {
            let mut new: Vec<_> = self.0.iter().rev().map(|d| d * cur).rev().collect();
            new.extend(vec![0; *tens as usize]);
            *tens += 1;
            Some(new)
        });
        mult.fold(BigUInt::default(), |a, b| a + Self(b))
    }
}

impl Sum for BigUInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, big| sum + big)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn palindrome_test() {
        assert!(BigUInt::from("9").is_palindrome());
        assert!(BigUInt::from("99").is_palindrome());
        assert!(BigUInt::from("909").is_palindrome());
        assert!(BigUInt::from("9009").is_palindrome());
        assert!(BigUInt::from("90109").is_palindrome());
        assert!(!BigUInt::from("91109").is_palindrome());
    }
}
