use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Clone, PartialEq)]
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

impl Add<BigUInt> for BigUInt {
    type Output = Self;

    fn add(self, rhs: BigUInt) -> Self::Output {
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
