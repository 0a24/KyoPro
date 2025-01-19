
// use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, HashSet};


fn dfs(g: &HashMap<String, HashSet<String>>, vis: &mut HashSet<&String>, fin: &mut HashSet<&String>, c: String) {
    vis.insert(&c);

    match g.get(&c) {
        None => (),
        Some(v) => {
            for i in v.iter() {
                if !vis.contains(i) {
                    dfs(g, vis, fin, i.to_string());
                }
            }        
        }
    }
    fin.insert(&c);
    return 
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [[char; 4]; m],
    }

    let mut g: HashMap<String, HashSet<String>> = HashMap::new();
    println!("{} {}", n, m);
    
    for i in 0..n {
        g.entry(i.to_string()+"R")
            .or_insert_with(HashSet::new)
            .insert(i.to_string()+"B");
        g.entry(i.to_string()+"B")
            .or_insert_with(HashSet::new)
            .insert(i.to_string()+"R");
    }

    for v in abcd.iter() {
        let l1: String = v[0..2].iter().collect::<String>();
        let r1: String = v[2..4].iter().collect::<String>();
        g.entry(l1)
            .or_insert_with(HashSet::new)
            .insert(r1);

         let l2: String = v[0..2].iter().collect::<String>();
         let r2: String = v[2..4].iter().collect::<String>();
         g.entry(r2)
            .or_insert_with(HashSet::new)
            .insert(l2);
    }

    let mut vis: HashSet<&String> = HashSet::new();
    let mut fin: HashSet<&String> = HashSet::new();
    for i in 0..n {
        let k: String = i.to_string()+"R";
        // let out: = 
        dfs(&g, &mut vis, &mut fin, k);

    }


    println!("{:?}", g);


}
