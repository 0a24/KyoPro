
use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }
    
    let mut s: i32 = 0;
    for i in 0..n {
        s += (a[i]-b[i]).abs();
    }
    
    if (k-s)>=0 && (k-s)%2==0 { println!("Yes") }
    else { println!("No") }
}
