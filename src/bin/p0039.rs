use project_euler::right_triangles_of_perimeter_p;

fn main() {
    let lens = (0..=1000)
        .map(|p| right_triangles_of_perimeter_p(p).len())
        .collect::<Vec<_>>();
    let max = lens.iter().max().unwrap();
    let r = lens.iter().position(|l| l == max);
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn right_triangles_of_perimeter_p_test() {
        assert_eq!(
            right_triangles_of_perimeter_p(120)
                .into_iter()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([(48, 20, 52), (45, 24, 51), (40, 30, 50)])
        );
    }
}
