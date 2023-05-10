fn multiple_of_3_or_5(x: &u64) -> bool {
    (x % 3 == 0) || (x % 5 == 0)
}

fn main() {
    let s: u64 = (1_u64..1000)
        .filter(multiple_of_3_or_5)
        .sum();
    println!("{}", s);
}
