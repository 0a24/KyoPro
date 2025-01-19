use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        _w: i32,
        mat: [Chars; h],
    }

    let mut n: i32 = 0;

    for s in &mat {
        for c in s {
            if c.to_string() == "#"{
                n += 1;
            }
        }
    }
    println!("{}", n);
}


