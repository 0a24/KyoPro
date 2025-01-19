use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    let mut ans: u128 = 0;
    let mut keta: u128 = 1;
    s.reverse();
    for c in s.iter() {
        ans += (*c as u128 - 64)*keta;
        keta *= 26;
    }
    println!("{}", ans);
}
