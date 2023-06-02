use project_euler::binomail_distribution;

use std::time::Instant;

fn main() {
    let s = Instant::now();
    let r = (1..=100)
        .map(|n| {
            (1..=n)
                .filter(|r| binomail_distribution(&n, r) >= 1_000_000)
                .count() as u64
        })
        .sum::<u64>();
    println!("{:?}", r);
    println!("{:?}", s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binomail_distribution_test() {
        assert_eq!(binomail_distribution(&4, &2), 6);
        assert_eq!(binomail_distribution(&5, &3), 10);
        assert_eq!(binomail_distribution(&23, &10), 1144066);
        assert_eq!(binomail_distribution(&40, &27), 12_033_222_880);
        assert_eq!(binomail_distribution(&68, &23), 801_957_983_888_792_640);
        assert_eq!(binomail_distribution(&68, &34), 28_453_041_475_240_576_740);
        assert_eq!(
            binomail_distribution(&100, &50),
            100_891_344_545_564_193_334_812_497_256
        );
    }
}
