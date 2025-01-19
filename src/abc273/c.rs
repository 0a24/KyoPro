use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    
    let mut map: HashMap<u32, u32> = HashMap::new();

    for v in a.iter() {
        match map.get(&v) {
            None => {
                map.insert(*v, 1);
            } 
            Some(c) => {
                map.insert(*v, c+1);
            }
        }
    }

    let mut vs: Vec<u32> = a;
    vs.sort();
    let mut tmp: u32 = 10u32.pow(9) + 1;
    for v in vs.iter().rev() {
        if *v==tmp { () }
        else {
            match map.get(&v) {
                Some(c) => {
                    println!("{}", c);
                }
                None => ()
            }
        }
        tmp = *v;
    }

    let l = n - map.len();    
    for _ in 0..l {
        println!("{}", 0);
    }
}
