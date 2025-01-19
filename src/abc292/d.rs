use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for &(i, j) in &edges {
        uf.union(i, j);
    }

    let mut vs: Vec<usize> = vec![0; n];
    let mut es: Vec<usize> = vec![0; n];

    for i in 0..n {
        vs[uf.find(i)] += 1;
    }
    for &(i, _) in &edges {
        es[uf.find(i)] += 1;
    }

    let ans: bool = vs==es;
    println!("{}", ["No", "Yes"][ans as usize]);
}
