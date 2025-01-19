use proconio::input;
use std::collections::{HashMap, HashSet};

fn dfs(g: &HashMap<usize, HashSet<usize>>, vis: &mut HashSet<usize>, c: usize) {
    vis.insert(c);

    match g.get(&c) {
        None => (),
        Some(v) => {
            for &i in v.iter() {
                if !vis.contains(&i) {
                    dfs(g, vis, i);
                }
            }        
        }
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut hmap: HashMap<usize, HashSet<usize>> = HashMap::new();

    for _i in 0..n {
        input! {
            t: (usize, usize),
        }
        hmap.entry(t.0)
            .or_insert_with(HashSet::new)
            .insert(t.1);
        hmap.entry(t.1)
            .or_insert_with(HashSet::new)
            .insert(t.0);
    }

    // let mut vis: Vec<bool> = vec![false; (10u32.pow(9)+1) as usize];
    let mut vis: HashSet<usize> = HashSet::new();
    dfs(&hmap, &mut vis, 1);

    let mut ans: usize = 1;
    for &v in vis.iter() {
        ans = std::cmp::max(ans, v)
    }
    println!("{:?}", hmap);
    println!("{}", ans);
}
