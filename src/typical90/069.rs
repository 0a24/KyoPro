use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    const M: u64 = 1_000_000_000 + 7;

    let mut ans: u64 = 1;
    if k>=n {
        ans *= k * (k-1) % M;
        for _ in 2..n {
            ans = ans * (k-2) % M;
        }
    } else {
        ()
    }

    // let mut cs: Vec<u64> = vec![0; n as usize];
    // i = 0;
    // cs[i] = n;
    // cs[i+1] = n-1;
    // cs[i+2] = n-2;
    
    println!("{}", ans);

}
