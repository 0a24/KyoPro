use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    
    let mut vis: Vec<bool> = vec![false; n];
    for id in 0..n {
        if !vis[id] {
            vis[a[id]] = true;
        }
    }

    let mut out: Vec<usize> = vec![];
    let mut id: usize = 1;
    let mut cnt: usize = 0;
    for i in vis.iter() {
        if !i {
            out.push(id);
            cnt += 1;
        }
        id += 1;
    }
    println!("{}", cnt);
    for v in out.iter() {
        print!("{} ", v);
    }
    println!();
}
