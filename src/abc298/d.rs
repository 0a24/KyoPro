use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut dq: VecDeque<i64> = VecDeque::new();
    dq.push_back(1);
    let mut res: i64 = 1;
    let n : i64 = 998244353;
    let mut ten_pow: Vec<i64> = vec![0; 6*10u32.pow(5) as usize + 1];
    ten_pow[0] = 1;
    for i in 1..ten_pow.len() {
        let tmp: i64 = (ten_pow[i-1] as i64 * 10) % n as i64;
        ten_pow[i] = tmp;
    }
    // for i in 0..10 {
    //     println!("{} {}",i, ten_pow[i]);
    // }
    // println!();

    for _ in 0..q {
        input! { qu: usize }
        match qu {
            1 => {
                input! { x: i64 }
                dq.push_back(x);
                res = (res*10 + x) % n;
                // println!("1 {}", res);
            },
            2 => {
                let l: usize = dq.len();
                let t: i64 = dq.pop_front().unwrap();
                // println!("2 {} {}*{}", res, t, ten_pow[l-1]);
                res -= t * ten_pow[l-1];
                // println!("2 {}", res);
                res %= n;
                // println!("2 {}", res);
                if res < 0 { res += n }
                // println!("2 {}", res);
            },
            3 => {
                println!("{}", res);
            },
            _ => (),
        }
    }

// println!("{:?}", dq);
}
