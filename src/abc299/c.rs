
use std::vec;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s0: Chars,
    }

    let mut s: Vec<char> = vec!['|'; n+2];
    for i in 1..n+1 {
        s[i] = s0[i-1];
    }

    let mut x_max: i32 = -1;
    let mut x: i32 = 0;
    let mut idx: usize = 0;

    for (i,&v) in s.iter().enumerate() {
        if v=='o' {
            x += 1;
        } else {
            if x>x_max && (v=='-' || s[idx]=='-') {
                x_max = x;
            }
            idx = i;
            x = 0;
        }
    }

    if x_max<=0 { println!("-1"); }
    else { println!("{}", x_max); }
}