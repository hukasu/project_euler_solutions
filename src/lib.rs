use std::collections::HashMap;

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

/// Add two lists of digits.
pub fn add_digit_lists(lhs: Vec<u8>, rhs: Vec<u8>) -> Vec<u8> {
    let mut l = lhs.into_iter().rev();
    let mut r = rhs.into_iter().rev();

    let mut carry = 0;
    let mut out = vec![];
    loop {
        match (l.next(), r.next()) {
            (Some(lv), Some(rv)) => {
                let add = lv + rv + carry;
                carry = add / 10;
                out.push(add % 10);
            },
            (Some(lv), None) => {
                let add = lv + carry;
                carry = add / 10;
                out.push(add % 10);
            },
            (None, Some(rv)) => {
                let add = rv + carry;
                carry = add / 10;
                out.push(add % 10);
            },
            (None, None) => {
                if carry > 0 {
                    out.push(carry);
                }
                break;
            }
        }
    }
    out.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn prime_factor_test() {
        assert_eq!(get_prime_factors(2), vec![]);
        assert_eq!(get_prime_factors(3), vec![]);
        assert_eq!(get_prime_factors(4), vec![2]);
        assert_eq!(get_prime_factors(5), vec![]);
        assert_eq!(get_prime_factors(6), vec![2, 3]);
        assert_eq!(get_prime_factors(13195), vec![5, 7, 13, 29]);
    }

    #[test]
    fn factor_test() {
        assert_eq!(get_factors(2), vec![]);
        assert_eq!(get_factors(3), vec![]);
        assert_eq!(get_factors(4), vec![2]);
        assert_eq!(get_factors(5), vec![]);
        assert_eq!(get_factors(6), vec![2, 3]);
        assert_eq!(get_factors(9), vec![3]);
        assert_eq!(get_factors(12), vec![2, 3, 4, 6]);
    }

    #[test]
    fn prime_factor_frequencies_test() {
        assert_eq!(get_prime_factors_frequencies(2), HashMap::from([(2, 1)]));
        assert_eq!(get_prime_factors_frequencies(3), HashMap::from([(3, 1)]));
        assert_eq!(get_prime_factors_frequencies(4), HashMap::from([(2, 2)]));
        assert_eq!(get_prime_factors_frequencies(5), HashMap::from([(5, 1)]));
        assert_eq!(
            get_prime_factors_frequencies(6),
            HashMap::from([(2, 1), (3, 1)])
        );
        assert_eq!(get_prime_factors_frequencies(16), HashMap::from([(2, 4)]));
        assert_eq!(
            get_prime_factors_frequencies(20),
            HashMap::from([(2, 2), (5, 1)])
        );
    }

    #[test]
    fn palindrome_test() {
        assert!(is_palindrome(9));
        assert!(is_palindrome(99));
        assert!(is_palindrome(909));
        assert!(is_palindrome(9009));
        assert!(is_palindrome(90109));
    }

    #[test]
    fn multiple_of_all_test() {
        assert_eq!(smallest_multiple_of_all_through_x(10), 2520);
    }

    #[test]
    fn sum_of_squares_test() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn square_of_sum_test() {
        assert_eq!(square_of_sum(10), 3025);
    }

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(4), 7);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(6), 13);
    }
}
