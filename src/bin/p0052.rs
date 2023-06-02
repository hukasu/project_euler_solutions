use project_euler::are_permutations;

fn main() {
    let r = (1..1_000_000)
        .filter(|n| are_permutations(n, &(2 * n)))
        .filter(|n| are_permutations(n, &(3 * n)))
        .filter(|n| are_permutations(n, &(4 * n)))
        .filter(|n| are_permutations(n, &(5 * n)))
        .find(|n| are_permutations(n, &(6 * n)));
    println!("{:?}", r);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn are_permutations_test() {
        assert!(are_permutations(&125874, &251748));
    }
}
