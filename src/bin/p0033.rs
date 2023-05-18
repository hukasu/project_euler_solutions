use project_euler::get_prime_factors_frequencies;

fn main() {
    let (num, den) = (0..=9)
        .flat_map(|num| {
            (1..=9)
                .map(|den| (num as f64, den as f64))
                .collect::<Vec<_>>()
        })
        .flat_map(|(num, den)| {
            (0..=9)
                .flat_map(|x| {
                    let others = vec![
                        (num + (x as f64 * 10.), den + (x as f64 * 10.)),
                        (num + (x as f64 * 10.), (den * 10.) + x as f64),
                        ((num * 10.) + x as f64, den + (x as f64 * 10.)),
                        ((num * 10.) + x as f64, (den * 10.) + x as f64),
                    ];
                    others.into_iter().map(|o| ((num, den), o))
                })
                .collect::<Vec<_>>()
        })
        .filter(|((num, _), _)| num > &0.)
        .filter(|((num, den), _)| (num / den) < 1.)
        .filter(|(_, (num, den))| num >= &10. && den >= &10.)
        .filter(|((num, _), (vsnum, _))| {
            !matches!(
                (vsnum / num).partial_cmp(&10.),
                Some(std::cmp::Ordering::Equal)
            )
        })
        .filter(|((num, den), (vsnum, vsden))| {
            matches!(
                (num / den).partial_cmp(&(vsnum / vsden)),
                Some(std::cmp::Ordering::Equal)
            )
        })
        .map(|(_, (num, den))| (num as u64, den as u64))
        .reduce(|(num, den), (nnum, nden)| (num * nnum, den * nden))
        .unwrap();
    let num_freq = get_prime_factors_frequencies(num);
    let den_freq = get_prime_factors_frequencies(den)
        .into_iter()
        .map(|(k, v)| k.pow(v.saturating_sub(*num_freq.get(&k).unwrap_or(&0)) as u32))
        .product::<u64>();
    println!("{:?}", den_freq);
}
