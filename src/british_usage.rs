#[derive(Debug, PartialEq)]
/// The text representation of numerals from 0 through 1000.
///
/// # Panic
/// Panics for values over 1000.
pub struct BritishUsage(pub String, u64);

impl BritishUsage {
    pub fn char_count(&self) -> u64 {
        self.0
            .chars()
            .map(|c| if c.is_ascii_lowercase() { 1 } else { 0 })
            .sum()
    }

    fn single_digit(x: u64) -> String {
        match x {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => panic!("Not single digit."),
        }
        .to_owned()
    }

    fn teens(x: u64) -> String {
        match x {
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!("Not in the teens (10..=19)."),
        }
        .to_owned()
    }

    fn tens(x: u64) -> String {
        match x {
            1 => "ten",
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => panic!("Not a prefix for a decimal place."),
        }
        .to_owned()
    }
}

impl From<u64> for BritishUsage {
    fn from(value: u64) -> Self {
        if value == 0 {
            Self("zero".to_owned(), value)
        } else {
            let mut remaining = value;
            let mut list = vec![];
            while remaining > 0 {
                match remaining {
                    x @ 1..=9 => {
                        remaining = 0;
                        list.push(Self::single_digit(x))
                    }
                    x @ 10..=19 => {
                        remaining = 0;
                        list.push(Self::teens(x))
                    }
                    x @ 20..=99 => {
                        let tens = x / 10;
                        let remain = x % 10;
                        let text = if remain == 0 {
                            Self::tens(tens)
                        } else {
                            vec![Self::tens(tens), "-".to_owned(), Self::single_digit(remain)]
                                .join("")
                        };
                        remaining = 0;
                        list.push(text)
                    }
                    x @ 100..=999 => {
                        let hundreds = x / 100;
                        let remain = x % 100;
                        let mut text = vec![Self::single_digit(hundreds), "hundred".to_owned()];
                        if remain > 0 {
                            text.extend(vec!["and".to_owned()]);
                        };
                        remaining = remain;
                        list.push(text.join(" "))
                    }
                    1000 => {
                        remaining = 0;
                        list.push("one thousand".to_owned())
                    }
                    _ => todo!(),
                }
            }
            Self(list.join(" "), value)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn british_usage_from_u64() {
        assert_eq!(BritishUsage::from(0), BritishUsage("zero".to_owned(), 0));
        assert_eq!(BritishUsage::from(1), BritishUsage("one".to_owned(), 1));
        assert_eq!(BritishUsage::from(2), BritishUsage("two".to_owned(), 2));
        assert_eq!(BritishUsage::from(3), BritishUsage("three".to_owned(), 3));
        assert_eq!(BritishUsage::from(4), BritishUsage("four".to_owned(), 4));
        assert_eq!(BritishUsage::from(5), BritishUsage("five".to_owned(), 5));
        assert_eq!(BritishUsage::from(6), BritishUsage("six".to_owned(), 6));
        assert_eq!(BritishUsage::from(7), BritishUsage("seven".to_owned(), 7));
        assert_eq!(BritishUsage::from(8), BritishUsage("eight".to_owned(), 8));
        assert_eq!(BritishUsage::from(9), BritishUsage("nine".to_owned(), 9));
        assert_eq!(BritishUsage::from(10), BritishUsage("ten".to_owned(), 10));
        assert_eq!(
            BritishUsage::from(11),
            BritishUsage("eleven".to_owned(), 11)
        );
        assert_eq!(
            BritishUsage::from(12),
            BritishUsage("twelve".to_owned(), 12)
        );
        assert_eq!(
            BritishUsage::from(13),
            BritishUsage("thirteen".to_owned(), 13)
        );
        assert_eq!(
            BritishUsage::from(14),
            BritishUsage("fourteen".to_owned(), 14)
        );
        assert_eq!(
            BritishUsage::from(15),
            BritishUsage("fifteen".to_owned(), 15)
        );
        assert_eq!(
            BritishUsage::from(16),
            BritishUsage("sixteen".to_owned(), 16)
        );
        assert_eq!(
            BritishUsage::from(17),
            BritishUsage("seventeen".to_owned(), 17)
        );
        assert_eq!(
            BritishUsage::from(18),
            BritishUsage("eighteen".to_owned(), 18)
        );
        assert_eq!(
            BritishUsage::from(19),
            BritishUsage("nineteen".to_owned(), 19)
        );
        assert_eq!(
            BritishUsage::from(20),
            BritishUsage("twenty".to_owned(), 20)
        );
        assert_eq!(
            BritishUsage::from(21),
            BritishUsage("twenty-one".to_owned(), 21)
        );
        assert_eq!(
            BritishUsage::from(32),
            BritishUsage("thirty-two".to_owned(), 32)
        );
        assert_eq!(
            BritishUsage::from(43),
            BritishUsage("forty-three".to_owned(), 43)
        );
        assert_eq!(
            BritishUsage::from(54),
            BritishUsage("fifty-four".to_owned(), 54)
        );
        assert_eq!(
            BritishUsage::from(65),
            BritishUsage("sixty-five".to_owned(), 65)
        );
        assert_eq!(
            BritishUsage::from(76),
            BritishUsage("seventy-six".to_owned(), 76)
        );
        assert_eq!(
            BritishUsage::from(87),
            BritishUsage("eighty-seven".to_owned(), 87)
        );
        assert_eq!(
            BritishUsage::from(98),
            BritishUsage("ninety-eight".to_owned(), 98)
        );
        assert_eq!(
            BritishUsage::from(101),
            BritishUsage("one hundred and one".to_owned(), 101)
        );
        assert_eq!(
            BritishUsage::from(111),
            BritishUsage("one hundred and eleven".to_owned(), 111)
        );
        assert_eq!(
            BritishUsage::from(190),
            BritishUsage("one hundred and ninety".to_owned(), 190)
        );
        assert_eq!(
            BritishUsage::from(191),
            BritishUsage("one hundred and ninety-one".to_owned(), 191)
        );
        assert_eq!(
            BritishUsage::from(206),
            BritishUsage("two hundred and six".to_owned(), 206)
        );
    }
}
