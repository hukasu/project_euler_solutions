use project_euler::get_prime_factors_frequencies;

fn main() {
    let r = (1..20)
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
        });
    println!("{r:?}");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

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
}
