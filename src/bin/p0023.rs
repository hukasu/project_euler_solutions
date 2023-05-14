use std::collections::BTreeSet;

use project_euler::NumberFactorSumClass;

const ABUNDANT_SUM_LIMIT: u64 = 28123;

fn main() {
    let abundant_numbers: Vec<_> = (1..=ABUNDANT_SUM_LIMIT)
        .filter(|n| {
            matches!(
                NumberFactorSumClass::get_class(n),
                NumberFactorSumClass::Abundant
            )
        })
        .collect();
    let sums_of_abundants: BTreeSet<u64> = abundant_numbers
        .iter()
        .flat_map(|l| abundant_numbers.iter().map(|r| l + r).collect::<Vec<_>>())
        .collect();
    let up_to_28123 = (1..=ABUNDANT_SUM_LIMIT).collect::<BTreeSet<u64>>();
    let not_sums = up_to_28123
        .difference(&sums_of_abundants)
        .collect::<Vec<_>>();
    println!("{:?}", not_sums.into_iter().sum::<u64>());
}
// 636852180
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_factor_sum_class_test() {
        assert!(matches!(
            NumberFactorSumClass::get_class(&28),
            NumberFactorSumClass::Perfect
        ));
        assert!(matches!(
            NumberFactorSumClass::get_class(&12),
            NumberFactorSumClass::Abundant
        ));
        for n in 1..12 {
            assert!(matches!(
                NumberFactorSumClass::get_class(&n),
                NumberFactorSumClass::Deficient | NumberFactorSumClass::Perfect
            ));
        }
    }
}
