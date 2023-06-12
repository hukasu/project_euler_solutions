fn main() {
    let r = (1_u64..10)
        .map(|x| {
            (1_u32..50)
                .map(|e| (e, x.saturating_pow(e)))
                .filter(|(e, pow)| pow.ilog10() + 1 == *e)
                .count()
        })
        .sum::<usize>()
        + 1; // Including 0^1
    println!("{}", r);
}

#[cfg(test)]
mod test {
    #[test]
    /// Tests if the result of the exponantiation has `n` digits
    fn test_filter_condition() {
        assert!(7_u64.saturating_pow(5_u32).ilog10() + 1 == 5);
        assert!(8_u64.saturating_pow(9_u32).ilog10() + 1 == 9);
    }
}
