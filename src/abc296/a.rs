use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut f = true;
    for i in 0..n-1 {
        if s[i]==s[i+1] {
            f = false;
            break;
        }
    }

    if f {
        println!("Yes")
    } else {
        println!("No")
    }
}
