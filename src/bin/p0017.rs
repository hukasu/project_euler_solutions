use project_euler::BritishUsage;

fn main() {
    let count: u64 = (1..=1000).map(|n| BritishUsage::from(n).char_count()).sum();
    println!("{count}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn british_usage_char_count() {
        assert_eq!(BritishUsage::from(342).char_count(), 23);
        assert_eq!(BritishUsage::from(115).char_count(), 20);
    }

    #[test]
    fn british_usage_char_count_from_range() {
        let count: u64 = (1..=5).map(|n| BritishUsage::from(n).char_count()).sum();
        assert_eq!(count, 19);
    }
}
