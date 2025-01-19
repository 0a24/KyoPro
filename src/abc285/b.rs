use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    
    let mut l: usize;

    for i in 1..n {
        l = 0;
        for k in 0..n-i {
            if s[k]!=s[k+i] {
                l += 1;
                if l==n-i {
                    println!("{}", l);
                }
            } else {
                println!("{}", l);
                break;
            }
        }
    }
}
