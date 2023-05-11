use project_euler::special_pythagorean_triplet;

fn main() {
    let spt = special_pythagorean_triplet(1000).unwrap();
    assert_eq!(spt.0 + spt.1 + spt.2, 1000);
    println!("{:?}", spt.0 * spt.1 * spt.2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn special_pythagorean_triplet_test() {
        assert_eq!(special_pythagorean_triplet(12).unwrap(), (3, 4, 5));
    }
}