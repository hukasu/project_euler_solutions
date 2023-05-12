use project_euler::grid_walk_path_count;

fn main() {
    println!("{}", grid_walk_path_count(20, 20));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_walk_path_count_test() {
        assert_eq!(grid_walk_path_count(2, 2), 6);
    }
}
