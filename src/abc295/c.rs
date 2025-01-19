
use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut hmap: HashMap<usize, u32> = HashMap::new();
    let mut hset: HashSet<usize> = HashSet::new();

    for &i in a.iter() {
        hset.insert(i);

        match hmap.get(&i) {
            None => {
                hmap.insert(i, 1);
            } 
            Some(v) => {
                hmap.insert(i, v+1);
            }
        }  
    }

    let mut ans: u32 = 0;
    for i in hset {
        match hmap.get(&i) {
            None => (),
            Some(v) => {
                ans += v/2;
            }
        }  
    }
    println!("{}", ans);
}