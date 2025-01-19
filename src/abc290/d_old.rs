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
        let mut x: u32 = 0;
        let mut hmap: HashMap<u32, bool> = HashMap::new();
        hmap.insert(x, true);
        for _ in 0..k-1 {
            x = (x+d) % n;

            loop {
                match hmap.get(&x) {
                    None => {
                        hmap.insert(x, true);
                        break;
                    } 
                    Some(false) => {
                        hmap.insert(x, true);
                        break;
                    }
                    Some(true) => {
                        x = (x+1) % n;
                    }
                }
            }
        }
        println!("{}", x);
    }
}
