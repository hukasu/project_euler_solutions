use std::collections::BTreeSet;

use project_euler::BigUInt;

fn main() {
    println!(
        "{}",
        (2_u64..=100)
            .flat_map(|a| (2_u64..=100)
                .map(|b| BigUInt::from(a).pow(b))
                .collect::<Vec<_>>())
            .collect::<BTreeSet<_>>()
            .len()
    );
}
