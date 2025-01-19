use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut s: Vec<u32> = vec![0; std::cmp::min(h,w)+1];

    for i in 0..h {
        for j in 0..w {
            if c[i][j]!='#' { () }
            else {
                let mut f: bool = true;
                let mut k = 0;
                while f {
                    k += 1;
                    if (i as i32 -k as i32)<0 || (j as i32 -k as i32 )<0 || (i+k)>=h || (j+k)>=w { break; }

                    if c[i-k][j-k]!='#' { f = false; }
                    if c[i-k][j+k]!='#' { f = false; }
                    if c[i+k][j-k]!='#' { f = false; }
                    if c[i+k][j+k]!='#' { f = false; }
                }
                // print!("{} ", k-1);
                s[k-1 as usize] += 1;
            }
        }
    }
    for i in 1..std::cmp::min(h,w)+1 {
        print!("{} ", s[i]);
    }
    println!("");
}