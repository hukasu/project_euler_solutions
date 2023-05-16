use std::collections::{BTreeSet, HashMap};

mod big_uint;
pub use big_uint::*;

mod british_usage;
pub use british_usage::*;

mod pyramid;
pub use pyramid::*;

/// Checks if number is multiple of either `3` or `5`.
pub fn multiple_of_3_or_5(x: &u64) -> bool {
    (x % 3 == 0) || (x % 5 == 0)
}

/// Checks if `f` is a factor of `x`.
pub fn is_factor(x: u64, f: u64) -> bool {
    x % f == 0
}

/// Checks if `f` is a prime factor of `x`.
pub fn is_prime_factor(x: u64, f: u64) -> bool {
    is_factor(x, f) && is_prime(f)
}

/// Returns the factors of a number, excluding 1 and itself.
pub fn get_factors(x: u64) -> Vec<u64> {
    if x > 2 {
        (2..=x.div_euclid(2)).filter(|f| is_factor(x, *f)).collect()
    } else {
        vec![]
    }
}

/// Returns the prime factors of a number, excluding 1 and itself.
pub fn get_prime_factors(x: u64) -> Vec<u64> {
    if x > 2 {
        [2_u64]
            .into_iter()
            .chain((3..=(x as f64).sqrt().ceil() as u64).step_by(2))
            .filter(|f| is_prime_factor(x, *f))
            .collect()
    } else {
        vec![]
    }
}

/// Returns a map of prime factors and the amount of times the number can be factored by it
pub fn get_prime_factors_frequencies(x: u64) -> HashMap<u64, u64> {
    [2_u64]
        .into_iter()
        .chain((3..=x).step_by(2))
        .fold((x, HashMap::new()), |(x, m), f| {
            let mut nm = m;
            let mut nx = x;
            while nx % f == 0 {
                nx = nx.div_euclid(f);
                let ff = nm.get(&f).unwrap_or(&0);
                nm.insert(f, ff + 1);
            }
            (nx, nm)
        })
        .1
}

/// Checks if a number is prime
pub fn is_prime(f: u64) -> bool {
    get_prime_factors(f).is_empty()
}

/// Checks if number is palindrome.
pub fn is_palindrome(x: u64) -> bool {
    let s = x.to_string();
    let s2: String = s.chars().rev().collect();
    s == s2
}

/// Get the smallest number that is multiple by all numbers from 1 through `x`.
pub fn smallest_multiple_of_all_through_x(x: u64) -> u64 {
    (1..=x)
        .map(get_prime_factors_frequencies)
        .reduce(|mut m, cm| {
            for (k, v) in cm.iter() {
                let ff = m.get(k).unwrap_or(&0);
                if v.ge(ff) {
                    m.insert(*k, *v)
                } else {
                    m.insert(*k, *ff)
                };
            }
            m
        })
        .unwrap_or_default()
        .iter()
        .fold(1_u64, |mut p, (k, v)| {
            p *= k.pow(*v as u32);
            p
        })
}

/// Get the sum of the squares from 1 through `x`.
pub fn sum_of_squares(x: u64) -> u64 {
    (1..=x).map(|x| x.pow(2)).sum()
}

/// Get the square of sum from 1 through `x`.
pub fn square_of_sum(x: u64) -> u64 {
    (1..=x).sum::<u64>().pow(2)
}

/// Get the N-th prime.
pub fn nth_prime(n: u64) -> u64 {
    (0..n).fold(1_u64, |prev, _| match prev {
        1 => 2,
        2 => 3,
        mut next => loop {
            next += 2;
            if is_prime(next) {
                break next;
            }
        },
    })
}

/// Get all primes up to `n`.
pub fn primes_up_to(x: u64) -> Vec<u64> {
    (0..1)
        .cycle()
        .scan(1_u64, |prev, _| match *prev {
            1 => {
                *prev = 2;
                Some(2)
            }
            2 => {
                *prev = 3;
                Some(3)
            }
            mut next => loop {
                next += 2;
                if next.gt(&x) {
                    return None;
                } else if is_prime(next) {
                    *prev = next;
                    break Some(next);
                }
            },
        })
        .collect()
}

/// Get the biggest product
pub fn biggest_adjacent_product(long_number: &str, adjacency: usize) -> u64 {
    let mut window = vec![0_u64; adjacency];
    let mut window_i = 0;
    let mut biggest = 0;
    for c in long_number.chars() {
        window[window_i] = c.to_digit(10).unwrap() as u64;
        window_i += 1;
        if window_i.ge(&window.len()) {
            window_i = 0;
        }
        let prod: u64 = window.iter().product();
        if prod.ge(&biggest) {
            biggest = prod;
        }
    }
    biggest
}

/// Check if triplet is a Pythagorean triplet.
/// a^2 + b^2 = c^2
pub fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

/// Finds a special Pythagorean triplet where `a + b + c = x`.
pub fn special_pythagorean_triplet(x: u64) -> Option<(u64, u64, u64)> {
    for c in (1..x).rev() {
        for b in (1..(x - c)).rev() {
            let a = x - (b + c);
            if is_pythagorean_triplet(a, b, c) {
                return Some((a, b, c));
            }
        }
    }
    None
}

/// Get the biggest product of adjacent values in a given direction (up, down, left, right or diagonally).
pub fn biggest_grid_adjacent_product<const N: usize>(
    grid: &[[u64; N]; N],
    adjacency: usize,
) -> u64 {
    let mut biggest = 0;
    for x in 0..N {
        for y in 0..N {
            let max = vec![
                grid_vertical_product(grid, x, y, adjacency),
                grid_horizontal_product(grid, x, y, adjacency),
                grid_diagonal_right_product(grid, x, y, adjacency),
                grid_diagonal_left_product(grid, x, y, adjacency),
            ]
            .into_iter()
            .max();
            if let Some(Some(m)) = max {
                if m.ge(&biggest) {
                    biggest = m;
                }
            }
        }
    }
    biggest
}

/// Get the product of the vertically adjacent values of a grid.
pub fn grid_vertical_product<const N: usize>(
    grid: &[[u64; N]; N],
    x: usize,
    y: usize,
    adjacency: usize,
) -> Option<u64> {
    (0..adjacency)
        .map(|offset| grid.get(y + offset).and_then(|inner| inner.get(x)))
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().product())
}

/// Get the product of the horizontally adjacent values of a grid.
pub fn grid_horizontal_product<const N: usize>(
    grid: &[[u64; N]; N],
    x: usize,
    y: usize,
    adjacency: usize,
) -> Option<u64> {
    (0..adjacency)
        .map(|offset| grid.get(y).and_then(|inner| inner.get(x + offset)))
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().product())
}

/// Get the product of the diagonally right adjacent values of a grid.
pub fn grid_diagonal_right_product<const N: usize>(
    grid: &[[u64; N]; N],
    x: usize,
    y: usize,
    adjacency: usize,
) -> Option<u64> {
    (0..adjacency)
        .map(|offset| grid.get(y + offset).and_then(|inner| inner.get(x + offset)))
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().product())
}

/// Get the product of the diagonally left adjacent values of a grid.
pub fn grid_diagonal_left_product<const N: usize>(
    grid: &[[u64; N]; N],
    x: usize,
    y: usize,
    adjacency: usize,
) -> Option<u64> {
    (0..adjacency)
        .map(|offset| {
            if x.ge(&offset) {
                grid.get(y + offset).and_then(|inner| inner.get(x - offset))
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().product())
}

/// Get the first Triagle number with N factors.
pub fn first_triangle_number_with_over_n_factors(factor_count: u64) -> u64 {
    let mut cont = true;
    (1_u64..=factor_count.pow(2))
        .scan(0, |prev, cur| {
            if cont {
                let s = *prev + cur;
                *prev = s;
                cont = get_factors(s).len().lt(&(factor_count as usize - 2));
                Some(s)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

/// Generates Collatz sequence of `x`.
pub fn collatz_sequence(x: u64) -> Vec<u64> {
    (0..1)
        .cycle()
        .scan(x, |x, _| {
            let old = *x;
            match x {
                0 => None,
                1 => {
                    *x = 0;
                    Some(1)
                }
                x if *x % 2 == 0 => {
                    *x /= 2;
                    Some(old)
                }
                x if *x % 2 == 1 => {
                    *x = 3 * *x + 1;
                    Some(old)
                }
                _ => panic!("Exhausted branches."),
            }
        })
        .collect()
}

/// Calculate number of paths on a grid walk with only `down`, or `right` moves.
pub fn grid_walk_path_count(w: usize, h: usize) -> u128 {
    let empty = vec![];
    let mut grid = vec![vec![1_u128; h + 1]; w + 1];
    for y in 0..=h {
        for x in 0..=w {
            if x + y == 0 {
                continue;
            }
            grid[x][y] = *grid
                .get(x.wrapping_sub(1))
                .unwrap_or(&empty)
                .get(y)
                .unwrap_or(&0)
                + *grid.get(x).unwrap().get(y.wrapping_sub(1)).unwrap_or(&0);
        }
    }

    grid[w][h]
}

/// Get the number of days in year.
/// ```
/// # use project_euler::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(1904), 366);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2001), 365);
/// assert_eq!(days_in_year(2004), 366);
/// ```
pub fn days_in_year(year: u64) -> u64 {
    let leap = if year % 400 == 0 {
        1
    } else if year % 100 == 0 {
        0
    } else if year % 4 == 0 {
        1
    } else {
        0
    };
    365 + leap
}

/// Count the number of days between 2 dates.
/// The dates should be input in ordinal format.
/// ```
/// # use project_euler::days_between_two_dates;
/// assert_eq!(days_between_two_dates((1900, 1), (1900, 1)), Ok(0));
/// assert_eq!(days_between_two_dates((1900, 1), (1900, 2)), Ok(1));
/// assert_eq!(days_between_two_dates((1900, 1), (1900, 7)), Ok(6));
/// assert_eq!(days_between_two_dates((1900, 1), (1900, 365)), Ok(364));
/// assert_eq!(days_between_two_dates((1900, 1), (1901, 1)), Ok(365));
/// assert!(days_between_two_dates((1993, 1), (1902, 1)).is_err());
/// assert!(days_between_two_dates((1900, 1), (1900, 366)).is_err());
/// assert_eq!(days_between_two_dates((1900, 1), (1904, 366)), Ok(1825));
/// ```
pub fn days_between_two_dates(
    (year1, day1): (u64, u64),
    (year2, day2): (u64, u64),
) -> Result<u64, String> {
    if day1 == 0 || day2 == 0 {
        Err("The first day in a year is the 1st.".to_owned())
    } else if year2.lt(&year1) || (year2.eq(&year1) && day2.lt(&day1)) {
        Err("The second date must be greater than the first.".to_owned())
    } else if days_in_year(year1).lt(&day1) || days_in_year(year2).lt(&day2) {
        Err("The day passed is greater then the number of days in the year.".to_owned())
    } else {
        Ok((year1..=year2)
            .map(|y| {
                let days_in_year = days_in_year(y);
                let discount = if y == year1 { day1 } else { 0 }
                    + if y == year2 {
                        days_in_year.saturating_sub(day2)
                    } else {
                        0
                    };
                days_in_year - discount
            })
            .sum())
    }
}

/// Count the number of weeks between 2 dates.
/// The dates should be input in ordinal format.
///
/// # Return
/// Returns a tuple with the first value being the number of weeks, and the second the number of extra days.
/// ```
/// # use project_euler::weeks_between_two_dates;
/// assert_eq!(weeks_between_two_dates((1900, 1), (1900, 1)), Ok((0, 0)));
/// assert_eq!(weeks_between_two_dates((1900, 1), (1900, 2)), Ok((0, 1)));
/// assert_eq!(weeks_between_two_dates((1900, 1), (1900, 7)), Ok((0, 6)));
/// assert_eq!(weeks_between_two_dates((1900, 1), (1900, 8)), Ok((1, 0)));
/// ```
pub fn weeks_between_two_dates(
    (year1, day1): (u64, u64),
    (year2, day2): (u64, u64),
) -> Result<(u64, u64), String> {
    let days = days_between_two_dates((year1, day1), (year2, day2))?;
    Ok((days / 7, days % 7))
}

/// Count number of Sundays on the first of the month.
pub fn sundays_on_the_first_of_the_month(date1: (u64, u64, u64), date2: (u64, u64, u64)) -> u64 {
    const MONTHS_WITH_30_DAYS: [u64; 4] = [4, 6, 9, 11];
    // Jan 1, 1900 is a Monday
    let mut current_date = (1900, 1, 7); // Initialized with the first Sunday of 1900
    let mut count = 0;
    while current_date < date2 {
        if current_date.2 == 1 && current_date > date1 {
            count += 1;
        }
        let month_limit = if current_date.1 == 2 {
            if days_in_year(current_date.0) == 366 {
                29
            } else {
                28
            }
        } else if MONTHS_WITH_30_DAYS.contains(&current_date.1) {
            30
        } else {
            31
        };
        let (next_month, next_day) = (
            current_date.1 + (current_date.2 + 7) / month_limit,
            (current_date.2 + 7) % month_limit,
        );
        let (next_year, next_month) = if next_month == 13 {
            (current_date.0 + 1, 1)
        } else {
            (current_date.0, next_month)
        };
        current_date = (next_year, next_month, next_day)
    }
    count
}

/// Check if a number is amicable.
///
/// An amicable number pair is where `sum_of_factors(a) = b` and `sum_of_factors(b) = a`, and `a != b`.
///
/// **Note**: The sum excludes the number itself, i.e. `sum_of_factors(6) = 1 + 2 + 3`
pub fn is_amicable_number(a: u64) -> bool {
    let b = get_factors(a).into_iter().sum::<u64>() + 1;
    let ra = get_factors(b).into_iter().sum::<u64>() + 1;
    a.ne(&b) && a.eq(&ra)
}

/// Calculate String score.
///
/// The score of a uppercase ASCII String is the sum of each letters score, where `A` = 1, `B` = 2, etc.
pub fn uppercase_ascii_string_score(s: &str) -> u64 {
    if s.chars().all(|c| c.is_ascii_uppercase()) {
        s.chars().map(|c| (c as u8 - b'A' + 1) as u64).sum::<u64>()
    } else {
        panic!("The String contains non uppercase ASCII characters.");
    }
}

/// Classifications of a number based on the sum of its factors.
///
/// A `Perfect` number has the sum of its factors equals to itself.
/// A `Deficient` number has the sum of its factors less than itself.
/// A `Abundant` number has the sum of its factors greater than itself.
pub enum NumberFactorSumClass {
    Abundant,
    Perfect,
    Deficient,
}

impl NumberFactorSumClass {
    /// Gets if number is Perfect, Abundant, or Deficient.
    pub fn get_class(x: &u64) -> Self {
        match (get_factors(*x).into_iter().sum::<u64>() + 1).cmp(x) {
            std::cmp::Ordering::Greater => Self::Abundant,
            std::cmp::Ordering::Equal => Self::Perfect,
            std::cmp::Ordering::Less => Self::Deficient,
        }
    }
}

/// Get the permutantions of all digits up to `n`.
pub fn permutations_of_digits_up_to_n(n: u64) -> BTreeSet<String> {
    if n >= 10 {
        panic!("Only single digits allowed.")
    }
    let mut res: BTreeSet<String> = (0..=n).map(|n| n.to_string()).collect();
    for _ in 0..n {
        res = res
            .iter()
            .flat_map(|v| {
                let already_in: BTreeSet<char> = v.chars().collect();
                let missing: BTreeSet<char> = (0..=n)
                    .map(|n| n.to_string().chars().next().unwrap())
                    .collect::<BTreeSet<char>>()
                    .difference(&already_in)
                    .cloned()
                    .collect();
                missing
                    .into_iter()
                    .map(|c| [v.clone(), c.to_string()].join(""))
                    .collect::<Vec<_>>()
            })
            .collect();
    }
    res
}

/// Preprocesses a string for the Knuth-Morris-Pratt string search algorithm.
pub fn knuth_morris_pratt_prepocessing(s: &str) -> Vec<isize> {
    let w: Vec<_> = s.chars().collect();
    println!("{:?}", w);
    let mut t = vec![-1; w.len()];

    let mut pos = 1;
    let mut cnd: isize = 0;

    while pos < w.len() {
        println!(
            "{} == {} {}",
            w[pos],
            w[cnd as usize],
            w[pos] == w[cnd as usize]
        );
        if w[pos] == w[cnd as usize] {
            t[pos] = t[cnd as usize];
        } else {
            t[pos] = cnd;
            while cnd >= 0 && w[pos] != w[cnd as usize] {
                cnd = t[cnd as usize];
            }
        }
        pos += 1;
        cnd += 1;
    }

    t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knuth_morris_pratt_prepocessing_test() {
        assert_eq!(
            knuth_morris_pratt_prepocessing("ABCDABD "),
            vec![-1, 0, 0, 0, -1, 0, 2, 0]
        );
        assert_eq!(
            knuth_morris_pratt_prepocessing("ABACABABC "),
            vec![-1, 0, -1, 1, -1, 0, -1, 3, 2, 0]
        );
        assert_eq!(
            knuth_morris_pratt_prepocessing("ABACABABA "),
            vec![-1, 0, -1, 1, -1, 0, -1, 3, -1, 3]
        );
        assert_eq!(
            knuth_morris_pratt_prepocessing("PARTICIPATE IN PARACHUTE "),
            vec![-1, 0, 0, 0, 0, 0, 0, -1, 0, 2, 0, 0, 0, 0, 0, -1, 0, 0, 3, 0, 0, 0, 0, 0, 0]
        );
    }
}
