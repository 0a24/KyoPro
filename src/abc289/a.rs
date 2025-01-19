use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    
    let mut t: Vec<char> = vec![];
    for v in s.iter() {
        if *v == '0' {
            t.push('1');
        } else {
            t.push('0');
        }
    }

    let u: String = t.iter().collect();
    println!("{}", u);
}
