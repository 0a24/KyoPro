use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }
    
    for i in (0..n).rev() {
        println!("{}", s[i]);
    }
}
