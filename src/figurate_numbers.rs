/// Check if `n` is a triangle number.
pub fn is_triangle_number(n: u64) -> bool {
    let delta = 1 + 8 * n;
    let x1 = (-1. + (delta as f64).sqrt()) / 2.;
    let x2 = (-1. - (delta as f64).sqrt()) / 2.;
    x1.fract() == 0.0 && x2.fract() == 0.0
}

/// Get the `n`th triangle number
#[inline(always)]
pub fn nth_triangle_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Get the `n`th square number
#[inline(always)]
pub fn nth_square_number(n: u64) -> u64 {
    n.pow(2)
}

/// Get the `n`th pentagon numbers.
#[inline(always)]
pub fn nth_pentagonal_number(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

/// Get the `n`th pentagon numbers.
#[inline(always)]
pub fn nth_hexagonal_number(n: u64) -> u64 {
    n * (2 * n - 1)
}

/// Get the `n`th heptagonal numbers.
#[inline(always)]
pub fn nth_heptagonal_number(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}

/// Get the `n`th pentagon numbers.
#[inline(always)]
pub fn nth_octagonal_number(n: u64) -> u64 {
    n * (3 * n - 2)
}
