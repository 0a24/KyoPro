use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        x: i32,
        p: [i32; n],
    }
    
    let mut idx: i32 = 1;
    for i in p{
        if i==x {
            println!("{}", idx);
        }
        idx += 1;
    }   
    
}
