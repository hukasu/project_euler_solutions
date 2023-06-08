use project_euler::BigUInt;

fn main() {
    let r = (1..1_000)
        .scan((BigUInt::from(1), BigUInt::from(2)), |(num, den), _| {
            let old_num = num.clone();
            *num = den.clone();
            *den = BigUInt::from(2) * den.clone() + old_num;
            Some((den.clone() + num.clone(), den.clone()))
        })
        .filter(|(num, den)| num.number_of_digits().gt(&den.number_of_digits()))
        .count();
    println!("{r:?}");
}
