use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut i1: usize = n;
    let mut i2: usize = n;
    let mut i3: usize = n;

    for (i,&v) in s.iter().enumerate() {
        match v {
            '|' => {
                if i1==n {
                    i1 = i;
                } else {
                    i3 = i;
                }
            },
            '*' => {
                i2 = i;
            }
            _ => (),
        }
    }

    if (i1<i2) && (i2<i3) {
        println!("in");
    } else {
        println!("out");
    }
}
