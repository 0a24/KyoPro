use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    
    let mut tmp: char;
    for i in 1..(s.len()/2+1){
        tmp = s[2*i-1-1];
        s[2*i-1-1] = s[2*i-1];
        s[2*i-1] = tmp;
    }

    let out: String = s.iter().collect();
    println!("{}", out);
}
