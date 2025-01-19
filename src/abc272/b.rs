use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: i32,
    }
    let mut f: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            k: usize,
            x: [usize; k],
        }
        for i in 0..k-1 {
            for j in i+1..k {
                f[x[i]-1][x[j]-1] = true;
                f[x[j]-1][x[i]-1] = true;
            }
        }    
    }

    let mut b = true;
    for i in 0..n {
        f[i][i] = true;
        b &= f[i].iter().all(|&x| x);
    }
    if b { println!("Yes"); }
    else { println!("No"); }

}
