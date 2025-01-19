use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut nt = 0;
    let mut na = 0;

    let mut w: usize = 0;

    for &v in s.iter() {
        if v=='T' {
            nt += 1;
        } else {
            na += 1;
        }
        if nt > na {
            w = 0;
        } else if nt < na {
            w = 1;
        }
    }

    println!("{}", ["T", "A"][w])
}
