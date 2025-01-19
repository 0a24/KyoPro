use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut a: i32 = -1;
    let mut b: i32 = -1;
    let mut c: i32 = -1;
    let mut d: i32 = -1;
    let mut chk_r;

    for (i,r) in s.iter().enumerate() {
        chk_r = false;
        for (j,&ch) in r.iter().enumerate() {
            if ch=='#' && a==-1{
                a = i as i32 +1;
                c = j as i32 +1;
            }
            if ch=='.' && d==-1 && a>0 {
                d = j as i32;
            }
            if ch=='#' {
                chk_r = true;
            }
        }
        if a>0 && !chk_r && b==-1{
            b = i as i32;
        }
        if a>0 && d==-1 {
            d = 10;
        }
    }
    if b==-1 {
        b = 10;
    }
    
    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
