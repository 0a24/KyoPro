use proconio::input;
// use std::collections::HashMap;

fn dfs(g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, c: usize) {
    vis[c] = true;
    for &i in g[c].iter() {
        if vis[i] == false {
            dfs(g, vis, i)
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..m {
        g[ab[i].0 - 1].push(ab[i].1 - 1);
        g[ab[i].1 - 1].push(ab[i].0 - 1);
    }

    let mut vis: Vec<bool> = vec![false; n];
    let mut cnt: u32 = 0;
    for i in 0..n {
        if vis[i] == false {
            dfs(&g, &mut vis, i);
            cnt += 1;
        }
    }

    println!("{}", m as u32 - (n as u32 - cnt));
}