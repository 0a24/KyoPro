
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut r: Vec<char> = vec!['?', 'U', 'P', 'C'];

    r[0] = s[0];

    let t: String = r.iter().collect();

    println!("{}", t);
}


