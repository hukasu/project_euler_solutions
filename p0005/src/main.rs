use methods::get_prime_factors_frequencies;

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
