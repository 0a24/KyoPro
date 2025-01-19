use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    // let n: usize = s.len();
    let mut idx: i32 = -2;

    // for (i,c) in s.iter().enumerate(){
    //     if c.to_string()=='a'.to_string() {
    for (i,&c) in s.iter().enumerate(){
        if c=='a' {
            idx = i as i32;
        }
    }
    println!("{}", idx+1);
}
