use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt: i32 = 0;

    for c in &s {
        if c.to_string() == "v"{
            cnt += 1;
        } else {
            cnt += 2;
        }
    }
    println!("{}", cnt);
}


