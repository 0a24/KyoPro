use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut f: bool = true;

    let mut idx_bs: Vec<i32> = vec![];
    let mut idx_rs: Vec<i32> = vec![];
    let mut idx_k: i32 = 0;
    for (i,&v) in s.iter().enumerate() {
        if v=='B' {
            idx_bs.push(i as i32);
        }
        if v=='R' {
            idx_rs.push(i as i32);
        }
        if v=='K' {
            idx_k = i as i32;
        }
    }

    let s: i32 = idx_bs.iter().sum();
    if s%2 != 1 {
        f = false;
    }
    
    if !(idx_rs[0]<idx_k && idx_k<idx_rs[1]) {
        f = false;
    }

    println!("{}", ["No", "Yes"][f as usize]);
}
