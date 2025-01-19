use proconio::input;

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

    if n-m!=1 {
        println!("No");
        return;
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..m {
        g[ab[i].0 - 1].push(ab[i].1 - 1);
        g[ab[i].1 - 1].push(ab[i].0 - 1);
    }

    let mut n1: u32 = 0;
    let mut i_start: usize = n+1;
    for i in 0..n {
        let i_g = &g[i];
        if i_g.len()<1 || 2<i_g.len() {
            println!("No");
            return;
        }
        if i_g.len()==1 {
            n1 += 1;
            i_start = i;
        }
    }
    if n1 != 2 {
        println!("No");
        return;
    }

    let mut vis: Vec<bool> = vec![false; n];
    dfs(&g, &mut vis, i_start);

    for v in vis {
        if !v {
            println!("No");
            return;
        }
    }
    println!("Yes");
}