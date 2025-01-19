use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        mut k: usize,
        s: Chars,
    }
    
    let mut t: Vec<char> = vec![];
    for &v in s.iter() {
        if k>0 && v=='o' {
            t.push('o');
            k -= 1;
        } else {
            t.push('x');
        }
    }
    
    let u: String = t.iter().collect();
    println!("{}", u);
}
