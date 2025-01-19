use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [[usize; 2]; n],
    }
    
    let mut dp: Vec<Vec<bool>> = vec![vec![false; x+1]; n+1];
    dp[0][0] = true;
    
    let mut a: usize;
    let mut b: usize;

    for i in 1..n+1 {
        a = ab[i-1][0];
        b = ab[i-1][1];
        for j in 0..x+1 {
            for b in 0..b+1 {
                if j < a*b { break; }
                dp[i][j] |= dp[i-1][j - a*b];
            }
        }            
    }

    if dp[n][x] { println!("Yes"); }
    else { println!("No"); }
}
