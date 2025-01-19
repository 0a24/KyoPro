use proconio::input;

use std::collections::BTreeSet;


fn main() {
    input! {
        _n: usize,
        q: usize,
    }

    let mut yobi_1: usize = 0;
    let mut called: BTreeSet<usize> = BTreeSet::new();

    let mut x: usize = 0;

    for _ in 0..q {
        input! { ev: usize }
        if ev==2 {
            input! { x_tmp: usize };
            x = x_tmp;
        }

        if ev==1 {
            yobi_1 += 1;
            called.insert(yobi_1);
        }
        if ev==2 {
            called.remove(&x);
        }
        if ev==3 {
            println!("{}", called.iter().next().unwrap());
        }
    }
}
