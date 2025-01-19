use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    
    for (i, v) in s.iter().enumerate() {
        if v.is_uppercase() {
            println!("{}", i+1);
            break;
        }
    }
}
