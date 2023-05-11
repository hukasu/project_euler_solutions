fn main() {
    let mut prev1 = 1;
    let mut prev2 = 1;
    let mut sum = 0;
    while prev2 < 4_000_000 {
        let n = prev1 + prev2;
        if n % 2 == 0 {
            sum += n;
        }
        prev1 = prev2;
        prev2 = n;
    }
    println!("{sum}");
}
