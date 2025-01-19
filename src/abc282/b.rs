use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    // println!("{:?}", s);

    let mut cnt: i32 = 0;
    let mut cnt_pair: usize;

    for i in 0..n{
        for j in i+1..n{
            cnt_pair = 0;
            for k in 0..m {
                // println!("{}", s[i][j]);
                if (s[i][k]=='o') || (s[j][k]=='o') { 
                    cnt_pair += 1; 
                }
            }
            // println!("{} {}", i,j);
            if cnt_pair==m {
                cnt += 1;
                // println!("HIT {} {}", i,j);
            }
        }
    }

    println!("{}", cnt);
}
