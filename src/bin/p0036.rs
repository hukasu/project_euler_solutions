use project_euler::is_palindrome;

fn main() {
    let r = (0..=1_000_000).filter(|n| is_palindrome(&n.to_string()) && is_palindrome(&format!("{:b}", n))).sum::<u64>();
    println!("{r:?}");
}
