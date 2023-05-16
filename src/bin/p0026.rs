use project_euler::UnitFraction;

fn main() {
    println!(
        "{:?}",
        (1..=1000)
            .enumerate()
            .map(|(i, d)| (
                i + 1,
                UnitFraction::new(d, 2_usize.pow(12)).get_cycle_length()
            ))
            .max_by(|(_, l), (_, r)| l.cmp(r))
    );
}
