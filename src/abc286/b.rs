use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    
    let mut ans: Vec<char> = vec![];

    for i in 0..n-1 {
        ans.push(s[i]);
        if s[i]=='n' && s[i+1]=='a' {
            ans.push('y');
        }
    }
    ans.push(s[n-1]);
    let s: String = ans.iter().collect();
    println!("{}", s);
}
