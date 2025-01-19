use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        hs: [i32; n],
    }

    let mut m = 0;
    let mut idx_m = 0;

    for (i, &h) in hs.iter().enumerate(){
        if h>m {
            m = h;
            idx_m = i+1;
        }
    }
    println!("{}", idx_m);
}
