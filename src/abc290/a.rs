use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [usize; m],
    }
    
    let mut ans: u32 = 0;
    for i in b.iter() {
        ans += a[i-1];
    }
    
    println!("{}", ans);
}
