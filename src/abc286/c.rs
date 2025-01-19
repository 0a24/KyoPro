use proconio::input;
use proconio::marker::Chars;


fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: Chars,
    }

    let mut j: usize;
    let mut cnt: u64;
    let mut cost: u64 = 1<<63;
    let mut cost_tmp: u64;

    for c in 0..n {
        cnt = 0;
        for i in 0..n/2 {
            j = n-1 - i;
            if s[(i+c)%n] == s[(j+c)%n] {
                cnt += 1;
            }
        }

        let need: u64 = (n as u64 - cnt*2)/2;

        cost_tmp = c as u64 * a + need * b;
        if cost_tmp < cost {
            cost = cost_tmp;            
        }
    }

    println!("{}", cost);

}