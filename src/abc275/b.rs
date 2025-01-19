use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        mut a: [i64; 6],
    }
    let p: i64 = 998244353;

    for i in 0..6 {
        a[i] = a[i] % p;
    }

    let mut b: i64 = 1;
    for i in 0..3 {
        b = (b * a[i]) % p;
    }
    let mut c: i64 = 1;
    for i in 3..6 {
        c = (c * a[i]) % p;
    }
    
    let mut ans: i64 = (b - c) % p;
    if ans<0 { ans += p; }
    println!("{}", ans);
}
