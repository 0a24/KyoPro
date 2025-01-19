
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_vec: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n: usize = s_vec.len();
    // println!("{:?}", s_vec);

    let mut h_vec: Vec<u32> = vec![0; n];
    h_vec[0] = 2u32.pow(s_vec[0]);
    for i in 1..n {
        h_vec[i] = (1<<s_vec[i]) ^ h_vec[i-1];
        // println!("{:010b} {}", h_vec[i], s_vec[i]);
    }

    let mut cnt: Vec<u32> = vec![0; 1024];
    let mut ans2: u64 = 0;

    cnt[0] += 1;
    for i in 0..n {
        ans2 += cnt[h_vec[i] as usize] as u64;

        cnt[h_vec[i] as usize] += 1;
    }

    let mut ans: u64 = 0;
    for &v in cnt.iter() {
        if v>=2 {
            ans += (v as u64*(v as u64-1)) / 2;
        }
    }

    // println!("{:?}", cnt);
    assert_eq!(ans, ans2);
    println!("{}", ans2);
}