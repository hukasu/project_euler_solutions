#[derive(Debug, Default, Clone, PartialEq)]
struct Wallet(pub [u8; 8]);

impl Wallet {
    /// Value of the wallet in pence
    fn value(&self) -> u64 {
        self.0
            .iter()
            .zip([200, 100, 50, 20, 10, 5, 2, 1].into_iter())
            .map(|(l, r)| *l as u64 * r)
            .sum()
    }
}

fn main() {
    let r = (0..1)
        .cycle()
        .scan(Wallet::default(), |w, _| {
            if w.0[0] == 1 {
                None
            } else {
                let mut nw = w.clone();
                nw.0[7] += 1;
                let nw = if nw.value() > 200 {
                    (1..8).rev().fold(nw, |mut w, i| {
                        if w.value() > 200 {
                            w.0[i - 1] += 1;
                            w.0[i] = 0;
                        }
                        w
                    })
                } else {
                    nw
                };
                *w = nw.clone();
                Some(nw)
            }
        })
        .filter(|w| w.value() == 200)
        .count();
    println!("{}", r)
}
