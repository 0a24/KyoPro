use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n],
    }

    let mut hmap: HashMap<u32, bool> = HashMap::new();
    for &v in a.iter() {
        hmap.insert(v, true);
    }

    let mut res: u32 = 0;
    for i in 0..10u32.pow(9)+1 {
        match hmap.get(&i) {
            None => {
                break;
            } 
            Some(true) => {
                res += 1;
                if res==k { break; }
            }
            Some(false) => {
                break;
            }
        }
    }
    println!("{}", res);
}