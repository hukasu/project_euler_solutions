use project_euler::BigUInt;

fn main() {
    println!(
        "{:?}",
        (0..1)
            .cycle()
            .scan((BigUInt::from(1), BigUInt::from(1)), |prev, _| {
                let next = prev.0.clone() + prev.1.clone();
                *prev = (prev.1.clone(), next.clone());
                Some(next)
            })
            .position(|f| f.number_of_digits().ge(&1000))
            .unwrap()
            + 3
    );
}
