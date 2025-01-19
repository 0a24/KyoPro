// https://atcoder.jp/contests/abc292/editorial/5942

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFind::new(n + 1);
    for &(i, j) in &edges {
        if !uf.union(i, j) {
            uf.union(i, n);
        }
    }
    let ans = n == m && (0..n).all(|i| uf.equiv(i, n));
    println!("{}", ["No", "Yes"][ans as usize]);
}
