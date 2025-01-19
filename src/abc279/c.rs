use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    
    let mut hmap: HashMap<Vec<char>, u32> = HashMap::new();

    for j in 0..w {
        let mut v: Vec<char> = vec![];
        for i in 0..h {
            v.push(s[i][j]);
        }
        match hmap.get(&v) {
            None => {
                hmap.insert(v, 1);
            } 
            Some(&k) => {
                hmap.insert(v, k+1);
            }
        }
    }
    
    for j in 0..w {
        let mut v: Vec<char> = vec![];
        for i in 0..h {
            v.push(t[i][j]);
        }
        match hmap.get(&v) {
            None => {
                println!("No");
                return;
            } 
            Some(&k) => {
                if k>0 {
                    hmap.insert(v, k-1);
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }
    
    println!("Yes");
    return;
}
