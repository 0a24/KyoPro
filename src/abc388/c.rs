
use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut c: [i32; n],
    }
    
    
    let mut c: usize = 0;

    for i in 0..n {
        c[i] -= (n + 1 - i) as i32;

    }

    println!("{}", c);
}


