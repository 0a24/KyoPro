use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        q: usize,
    }

    let mut hmap: HashMap<(usize,usize), bool> = HashMap::new();

    for _i in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }
        match t {
            1 => { hmap.insert((a,b), true); },
            2 => { hmap.insert((a,b), false); },
            3 => {
                let b_a: bool;
                let b_b: bool;

                match hmap.get(&(a,b)) {
                    Some(true) => b_a = true,
                    _ => b_a = false,
                }
                match hmap.get(&(b,a)) {
                    Some(true) => b_b = true,
                    _ => b_b = false,
                }

                if b_a & b_b {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => (),
        }
    }
}
