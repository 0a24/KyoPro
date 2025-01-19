use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        h: i32,
        w: i32,
    }

    if h==1 || w == 1 {
        println!("{}", std::cmp::max(h,w))
    } else {
        let i = (h+1)/2;
        let j =  (w+1)/2;
        println!("{}", i*j);    
    }
}
