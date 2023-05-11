use methods::multiple_of_3_or_5;

fn main() {
    let s: u64 = (1_u64..1000).filter(multiple_of_3_or_5).sum();
    println!("{}", s);
}
