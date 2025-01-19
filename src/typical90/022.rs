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
        c: u64,
    }
    let d = gcd(gcd(a,b),c);
    println!("{}", (a/d-1)+(b/d-1)+(c/d-1));
}
