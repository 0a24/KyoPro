
// use itertools::Itertools;
use proconio::input;


fn dfs(g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, fin: &mut Vec<bool>, c: usize, p: usize)->usize {
    if vis[c] {
        return 1
    }

    vis[c] = true;

    let mut res: usize = 2;
    for &v in g[c].iter() {
        if v == p {
            ()
        } else if fin[v] {
            res = 2;
        } else {
            if v == 0 {
                res = 0;
            } else {
                res = dfs(g, vis, fin, v, c);
            }        
        }
    }

    fin[c] = true;
    return res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [[char; 4]; m],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![0; 2]; 2*n+2];

    for i in 2..2*n+2 {
        if i%2 == 0 {
            g[i][0] = i+1;
        } else {
            g[i][0] = i-1;
        }
    }

    for v in abcd.iter() {
        let num1: usize = v[0].to_digit(10).unwrap() as usize;
        let num2: usize = v[2].to_digit(10).unwrap() as usize;
        
        if v[1] == 'R' && v[3] == 'R' {
            g[num1*2][1] = num2*2;
        } else if v[1] == 'R' && v[3] == 'B' {
            g[num1*2][1] = num2*2+1;         
        } else if v[1] == 'B' && v[3] == 'R' {
            g[num1*2+1][1] = num2*2;
        } else if v[1] == 'B' && v[3] == 'B' {
            g[num1*2+1][1] = num2*2+1;         
        }
    }

    println!("{:?}", g);

    let mut fin: Vec<bool> = vec![false; 2*n+2];
    let mut ring: usize = 0;
    let mut n_ring: usize = 0;
    for i in 2..2*n+2 {
        if !fin[i] {
            let mut vis: Vec<bool> = vec![false; 2*n+2];
            let res: usize;
            res = dfs(&g, &mut vis, &mut fin, i, 0);

            if res == 1 {
                ring += 1;
                println!("ring {} {}", i, ring);
            } else if res == 0 {
                n_ring += 1;
                println!("n_ring {} {}", i, n_ring);
            }
        }
    }
    
    println!("{} {}", ring, n_ring);
}
