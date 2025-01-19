use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u32,
            d: u32,
            k: u32,
        }
        println!("{} {} {}", n,d,k);

        let mut x: u32 = 0;

        let n_d = n / d;
        let nmodd = n % d;

        let k_nd = k / n_d;


        
        println!("{} {}", n_d, k_nd);
    }
}
