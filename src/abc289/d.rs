use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        mut b: [usize; m],
        x: usize,
    }
    
    let mut mapb: HashMap<usize,bool> = HashMap::new();
    for v in b.iter() {
        mapb.insert(*v, true);
    }

    let mut st: Vec<bool> = vec![false; x+1];
    st[0] = true;
    let mut t: usize;
    for i in 0..x+1 {
        if st[i] {
            for v in a.iter() {
                if i+v<x+1 {
                    t = i+v;
                    match mapb.get(&t) {
                        None => {
                            st[t] = true;
                        } 
                        Some(_) => {
                            ()
                        }
                    }
                }
            }
        }
    }

    if st[x] { println!("Yes"); }
    else { println!("No"); }
}
