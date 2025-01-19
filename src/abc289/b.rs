use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, c: usize) {
    vis[c] = true;
    for &i in g[c].iter() {
        if vis[i] == false {
            dfs(g, vis, i)
        }
    }
    print!("{} ", c+1);
}

fn main() {
    input! {n: usize, m: usize};
    input! {a: [usize; m]};

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];

    for i in a.iter() {
        g[i - 1].push(*i);
    }

    let mut vis: Vec<bool> = vec![false; n];
    for i in 0..n {
        if vis[i] == false {
            dfs(&g, &mut vis, i);
        }
    }
    println!("");
}