use project_euler::is_lychrel_number;

fn main() {
    let r = (1..10000).filter(|n| is_lychrel_number(n, 50)).count();
    println!("{r}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_lychrel_number_test() {
        assert!(!is_lychrel_number(&191, 50));
        assert!(!is_lychrel_number(&195, 50));
        assert!(is_lychrel_number(&196, 50));
        assert!(!is_lychrel_number(&197, 50));
        assert!(is_lychrel_number(&4994, 50));
    }
}
