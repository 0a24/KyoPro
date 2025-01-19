use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
    }
    
    let s_all: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut s = Vec::<String>::new();
    for i in 0..k{
        s.push(s_all[i].to_string())
    }

    println!("{}", s.concat());
}
