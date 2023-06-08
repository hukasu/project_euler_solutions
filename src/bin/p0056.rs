use project_euler::BigUInt;

fn main() {
    let r = (1..100)
        .map(|a| {
            let big_a = BigUInt::from(a);
            (1..100)
                .scan(BigUInt::from(1), |prod, _| {
                    *prod = prod.clone() * big_a.clone();
                    Some(prod.clone())
                })
                .map(|bui| BigUInt::sum_of_digits(&bui))
                .max()
                .unwrap()
        })
        .max();
    println!("{r:?}");
}
