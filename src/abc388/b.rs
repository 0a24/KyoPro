
use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [[u32; 2]; n],
    }
    
    for k in 1..d+1 {
        let mut m: u32 = 0;

        for i in 0..n {
            // println!("{:?}", tl[i]);
            let tmp = tl[i][0] * (tl[i][1] + k as u32);

            if tmp > m {
                m = tmp;
            }
        }
        println!("{}", m);
    }
}


