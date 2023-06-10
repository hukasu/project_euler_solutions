use std::{
    collections::{BTreeSet, HashMap},
    iter::{FlatMap, Skip, StepBy, Take},
    ops::{Add, Range, RangeInclusive},
};

mod big_uint;
pub use big_uint::*;

mod british_usage;
pub use british_usage::*;

mod figurate_numbers;
pub use figurate_numbers::*;

mod poker;
pub use poker::*;

mod pyramid;
pub use pyramid::*;

mod quadratic_formula;
pub use quadratic_formula::*;

mod unit_fraction;
pub use unit_fraction::*;

/// Checks if number is multiple of either `3` or `5`.
pub fn multiple_of_3_or_5(x: &u64) -> bool {
    (x % 3 == 0) || (x % 5 == 0)
}

/// Checks if `f` is a factor of `x`.
pub fn is_factor(x: u64, f: u64) -> bool {
    x % f == 0
}

/// Checks if a number is prime
pub fn is_prime(f: u64) -> bool {
    match f {
        2 | 3 => true,
        a if a <= 1 || (a % 2 == 0) || (a % 3 == 0) => false,
        a => !(5..=((a + 1) as f64).sqrt() as u64)
            .step_by(6)
            .any(|i| a % i == 0 || a % (i + 2) == 0),
    }
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

/// Returns the prime factors of a number.
pub fn get_prime_factors(x: u64) -> Vec<u64> {
    primes_up_to(x)
        .into_iter()
        .filter(|f| is_prime_factor(x, *f))
        .collect()
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

/// Checks if number is palindrome.
pub fn is_palindrome(s: &str) -> bool {
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
pub fn primes_up_to(limit: u64) -> BTreeSet<u64> {
    const CONDITION_1: [u64; 8] = [1, 13, 17, 29, 37, 41, 49, 53];
    const CONDITION_2: [u64; 4] = [7, 19, 31, 43];
    const CONDITION_3: [u64; 4] = [11, 23, 47, 59];

    let sqrt_limit = (limit as f64).sqrt() as u64 + 1;

    let sieve = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];
    let mut is_prime = vec![false; limit as usize + 1];

    // Condition 1
    (1..sqrt_limit)
        // Cartesian product of x E {1, 2, ...} and y E {1, 3, ...}
        .flat_map(|x| {
            (1..sqrt_limit)
                .step_by(2)
                .map(|y| (x, y))
                .collect::<Vec<_>>()
        })
        // Calculate n <- 4 * x^2 + y^2
        .map(|(x, y)| 4 * x.pow(2) + y.pow(2))
        .filter(|n| n <= &limit)
        .filter(|n| CONDITION_1.binary_search(&(n % 60)).is_ok())
        .for_each(|n| is_prime[n as usize] = !is_prime[n as usize]);

    // Condition 2
    (1..sqrt_limit)
        .step_by(2)
        // Cartesian product of x E {1, 3, ...} and y E {2, 4, ...}
        .flat_map(|x| {
            (2..sqrt_limit)
                .step_by(2)
                .map(|y| (x, y))
                .collect::<Vec<_>>()
        })
        // Calculate n <- 3 * x^2 + y^2
        .map(|(x, y)| 3 * x.pow(2) + y.pow(2))
        .filter(|n| n <= &limit)
        .filter(|n| CONDITION_2.binary_search(&(n % 60)).is_ok())
        .for_each(|n| is_prime[n as usize] = !is_prime[n as usize]);

    // Condition 3
    (2..sqrt_limit)
        // Cartesian product of x E {2, 3, ...} and y E {x - 1, x - 3, ..., 1}
        .flat_map(|x| {
            (1..sqrt_limit)
                .step_by(2)
                .scan((), |_, sub| if sub > x { None } else { Some((x, x - sub)) })
                .collect::<Vec<_>>()
        })
        // Calculate n <- 3 * x^2 - y^2
        .map(|(x, y)| 3 * x.pow(2) - y.pow(2))
        .filter(|n| n <= &limit)
        .filter(|n| CONDITION_3.binary_search(&(n % 60)).is_ok())
        .for_each(|n| is_prime[n as usize] ^= true);

    // Composites
    (0..sqrt_limit)
        // Cartesian product of w E {0, 1, ...} and x E `sieve`
        .flat_map(|w| sieve.iter().map(|x| (w, *x)).collect::<Vec<_>>())
        // Calculate n <- 60 * w + x
        .map(|(w, x)| 60 * w + x)
        .filter(|n| n >= &7)
        .filter(|n| n.pow(2) <= limit)
        .for_each(|n| {
            if is_prime[n as usize] {
                (0..sqrt_limit)
                    // Cartesian product of w E {0, 1, ...} and x E `sieve`
                    .flat_map(|w| sieve.iter().map(|x| (w, *x)).collect::<Vec<_>>())
                    // Calculate c <- n^2 * (60 * w + x)
                    .map(|(w, x)| n.pow(2) * (60 * w + x))
                    .filter(|c| c <= &limit)
                    .for_each(|c| is_prime[c as usize] = false)
            }
        });

    BTreeSet::from_iter(
        [2, 3, 5].into_iter().chain(
            (0..limit)
                .flat_map(|w| sieve.iter().map(|x| (w, *x)).collect::<Vec<_>>())
                .map(|(w, x)| 60 * w + x)
                .filter(|n| n >= &7)
                .filter(|n| n <= &limit)
                .filter(|n| is_prime[*n as usize]),
        ),
    )
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

/// Get the first Triangle number with N factors.
pub fn first_triangle_number_with_over_n_factors(factor_count: u64) -> u64 {
    (0..=factor_count.pow(2))
        .map(nth_triangle_number)
        .find(|t| get_factors(*t).len() as u64 >= (factor_count - 2))
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

type Spiral = FlatMap<
    StepBy<Range<u64>>,
    Take<Skip<StepBy<RangeInclusive<u64>>>>,
    Box<dyn FnMut(u64) -> Take<Skip<StepBy<RangeInclusive<u64>>>>>,
>;

/// Create a Spiral generator.
pub fn vertices_of_number_spiral(n: u64) -> Result<Spiral, String> {
    if n % 2 == 0 {
        Err("The sides of a Spiral have odd length.".into())
    } else {
        let shells_dimensions = (1_u64..n).step_by(2);
        let shell_vertices = |d: u64| {
            (d.pow(2)..=(d + 2).pow(2))
                .step_by((d + 1) as usize)
                .skip(1)
                .take(4)
        };
        Ok(shells_dimensions.flat_map(Box::new(shell_vertices)))
    }
}

/// Get the sum of diagonals on a spiral of side `n`.
pub fn spiral_diagonals_sum(n: u64) -> Result<u64, String> {
    Ok(vertices_of_number_spiral(n)?.sum::<u64>().add(1))
}

/// Get all circular shifts of a number `n`.
pub fn circular_shifts_of_a_number(n: u64) -> Vec<u64> {
    let mut t = n.to_string();
    let mut res = vec![n];
    loop {
        let head = t.remove(0);
        t.push(head);
        let nn = t.parse::<u64>().unwrap();
        if nn == n {
            break;
        } else {
            res.push(nn);
        }
    }
    res
}

/// Check if prime is left-to-right truncable.
pub fn left_truncable_prime(x: u64) -> bool {
    (0..1)
        .cycle()
        .scan(1, |p, _| {
            if x % 10_u64.pow(*p) == x {
                None
            } else {
                *p += 1;
                Some(x % 10_u64.pow(*p - 1))
            }
        })
        .all(is_prime)
}

/// Check if prime is left-to-right truncable.
pub fn right_truncable_prime(x: u64) -> bool {
    (0..1)
        .cycle()
        .scan(x, |x, _| {
            if *x / 10 > 0 {
                *x /= 10;
                Some(*x)
            } else {
                None
            }
        })
        .all(is_prime)
}

/// Check if prime is truncable
pub fn truncable_prime(x: u64) -> bool {
    static SINGLE_DIGIT: [u64; 4] = [2, 3, 5, 7];
    if is_prime(x) && !SINGLE_DIGIT.contains(&x) {
        left_truncable_prime(x) && right_truncable_prime(x)
    } else {
        false
    }
}

/// Collect digits present on number into a sorted list.
pub fn number_to_sorted_list_of_digits(n: &u64) -> Vec<char> {
    let mut d = n.to_string().chars().collect::<Vec<_>>();
    d.sort();
    d
}

/// Collect digits present on number into a set.
pub fn number_to_set_of_digits(n: &u64) -> BTreeSet<char> {
    n.to_string().chars().collect::<BTreeSet<_>>()
}

/// Check if list of numbers is pandigital.
///
/// A pandigital number is a number that has all digits from 1 through `n` exactly once, where `n` is the
/// length of characters of the string from the concatenation of the numbers in `ns`.
pub fn pandigital_numbers(ns: &[u64], start_at_zero: bool) -> bool {
    static ZERO_THROUGH_NINE: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    static ONE_THROUGH_NINE: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let cmp = if start_at_zero {
        ZERO_THROUGH_NINE.as_slice()
    } else {
        ONE_THROUGH_NINE.as_slice()
    };
    let d = ns
        .iter()
        .flat_map(number_to_sorted_list_of_digits)
        .collect::<Vec<_>>();
    if d.len() > cmp.len() {
        false
    } else {
        d.eq(&cmp[..d.len()])
    }
}

/// Compare if 2 numbers have the same digits.
pub fn are_permutations(lhs: &u64, rhs: &u64) -> bool {
    number_to_sorted_list_of_digits(lhs).eq(&number_to_sorted_list_of_digits(rhs))
}

/// Get all solutions for a right triangle of integer perimeter `p`.
pub fn right_triangles_of_perimeter_p(p: u64) -> Vec<(u64, u64, u64)> {
    let mut r = vec![];

    let mut a = p as f64;
    let mut b = 1.;

    while a >= b {
        let c = (a.powf(2.) + b.powf(2.)).sqrt();
        match (a + b + c).total_cmp(&(p as f64)) {
            std::cmp::Ordering::Less => b += 1.,
            std::cmp::Ordering::Equal => {
                r.push((a as u64, b as u64, c as u64));
                b += 1.;
            }
            std::cmp::Ordering::Greater => a -= 1.,
        }
    }

    r
}

/// Find sequencial numbers of length `window` that all have `factor` distinct prime factors.
///
/// # Return
/// Returns the first number in the sequency
pub fn sequencial_distinct_prime_factors(window: u64, factor: u64, stop: u64) -> Option<u64> {
    (1..stop)
        .map(|n| (n, get_prime_factors(n).len() as u64))
        .scan(0, |count, cur| {
            if count == &window {
                None
            } else if cur.1 == factor {
                *count += 1;
                Some(cur.0)
            } else {
                *count = 0;
                Some(cur.0)
            }
        })
        .last()
        .map(|d| d - (window - 1))
}

/// Calculate the power modulus.
pub fn power_modulus(x: u64, pow: u64, modulus: u64) -> u64 {
    (0..pow).fold(1, |prod, _| (prod * x) % modulus)
}

/// Generates a list of the sums of consecutives primes up to `threshold`.
///
/// The sequency with `skip` at `0` have the sums of \[2, 3, 5, ...\], `skip` at `1` has the sums
/// of \[3, 5, 7, ...\], and so on.
pub fn sum_of_consecutive_primes(threshold: u64, skip: usize) -> Vec<u64> {
    let primes = primes_up_to(threshold);
    primes
        .iter()
        .skip(skip)
        .scan(0_u64, |sum, cur| {
            *sum += cur;
            Some(*sum)
        })
        .collect()
}

/// Replace some digits of a number for another digit
///
/// ```
/// use project_euler::replace_digits;
/// assert_eq!(replace_digits(54321, 1001, 6), 56326);
/// assert_eq!(replace_digits(5432154321, 100101001, 7), 5732757327);
/// ```
pub fn replace_digits(n: u64, mask: u64, digit: u8) -> u64 {
    if digit >= 10 {
        panic!("Must be single digit.")
    };
    let str = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut mask = mask
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    while str.len() > mask.len() {
        mask.insert(0, 0);
    }
    str.iter()
        .zip(mask)
        .map(|(s, m)| {
            if m == 1 {
                digit.to_string()
            } else {
                s.to_string()
            }
        })
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

/// Get greatest common divisor.
pub fn greatest_common_divisor(lhs: &u128, rhs: &u128) -> u128 {
    if rhs == &0 {
        *lhs
    } else {
        greatest_common_divisor(rhs, &(lhs % rhs))
    }
}

/// Binomial distribution.
pub fn binomail_distribution(n: &u64, r: &u64) -> u128 {
    let nume = (*r as u128 + 1)..=(*n as u128);
    let deno = 1..=((n - r) as u128);
    let frac = nume
        .zip(deno)
        .fold((1_u128, 1_u128), |(fracnum, fracdeno), (n, d)| {
            let factor = greatest_common_divisor(&n, &d);
            let next = (fracnum * (n / factor), fracdeno * (d / factor));
            let factor = greatest_common_divisor(&next.0, &next.1);
            (next.0 / factor, next.1 / factor)
        });
    frac.0 / frac.1
}

/// Verifies if a number is a Lychrel Numbers.
pub fn is_lychrel_number(n: &u64, limit: u64) -> bool {
    !(0..limit)
        .scan(BigUInt::from(*n), |p, _| {
            *p += p.clone().reverse();
            Some(p.clone())
        })
        .any(|n| n.is_palindrome())
}

/// Verifies if the concatenation of 2 numbers `r` and `l`, on both orientations, (`rl` and `lr`) are primes.
pub fn is_pair_concatenation_prime(l: &u64, r: &u64) -> bool {
    is_prime(l * 10_u64.pow(r.ilog10() + 1) + r) && is_prime(r * 10_u64.pow(l.ilog10() + 1) + l)
}

/// Check if the last `n` digits of `l` are the first `n` digits of `r`.
pub fn cyclic_number(l: &u64, r: &u64, n: u32) -> bool {
    match l.ilog10() <= n || r.ilog10() <= n {
        true => false,
        false => l % 10_u64.pow(n) == r / 10_u64.pow(r.ilog10() - n + 1),
    }
}

/// Preprocesses a string for the Knuth-Morris-Pratt string search algorithm.
pub fn knuth_morris_pratt_prepocessing(s: &str) -> Vec<isize> {
    let w: Vec<_> = s.chars().collect();
    let mut t = vec![-1; w.len()];

    let mut pos = 1;
    let mut cnd: isize = 0;

    while pos < w.len() {
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
