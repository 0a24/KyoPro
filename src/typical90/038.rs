use proconio::input;
// use proconio::marker::Chars;

fn gcd(a: u64, b: u64) -> u64 {
    let r: u64 = a % b;
    if r == 0 { return b }
    else { return gcd(b,r) }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    
    let e: u64 = 1000_000_000_000_000_000;
    let g: u64 = gcd(a, b);
    
    if g==a || g==b {
        println!("{}", std::cmp::max(a,b));
    } else {
        let c: u64 = a / g;
        if e/c/b < 1  {
            println!("Large");
        } else {
            println!("{}", c*b);
        }
    }
}
