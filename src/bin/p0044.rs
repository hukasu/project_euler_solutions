use std::collections::BTreeSet;

use project_euler::nth_pentagonal_number;

fn main() {
    let pentagon = (1..10_000)
        .map(nth_pentagonal_number)
        .collect::<BTreeSet<_>>();
    let r = pentagon
        .iter()
        .flat_map(|l| pentagon.iter().zip(vec![l].into_iter().cycle()))
        .filter(|(l, r)| l > r)
        .filter(|(l, r)| pentagon.contains(&(*l + *r)) && pentagon.contains(&(*l - *r)))
        .map(|(l, r)| l - r)
        .min();
    println!("{r:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pentagon_number_test() {
        assert_eq!(nth_pentagonal_number(1), 1);
        assert_eq!(nth_pentagonal_number(2), 5);
        assert_eq!(nth_pentagonal_number(3), 12);
        assert_eq!(nth_pentagonal_number(4), 22);
        assert_eq!(nth_pentagonal_number(5), 35);
    }
}
