use project_euler::is_palindrome;

fn main() {
    'outer: for a in (100..=999).rev() {
        let dist_to_max = 999 - a;
        let a_range = a - dist_to_max..=999;
        let b_range = (a - dist_to_max..=999).rev();
        for (l, r) in a_range.zip(b_range) {
            let m = l * r;
            if is_palindrome(m) {
                println!("{l} * {r} = {m}");
                break 'outer;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn palindrome_test() {
        assert!(is_palindrome(9));
        assert!(is_palindrome(99));
        assert!(is_palindrome(909));
        assert!(is_palindrome(9009));
        assert!(is_palindrome(90109));
    }
}
