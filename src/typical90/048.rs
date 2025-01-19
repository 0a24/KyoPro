use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut ab: [(i32, i32); n],
    }

    let mut v: Vec<i32> = vec![0; n*2];
    for (i,t) in ab.iter().enumerate() {
        v[i] = t.1;
        v[i+n] = t.0 - t.1;
    }
    v.sort_by(|a, b| (-a).partial_cmp(&(-b)).unwrap());

    let mut ans: u64 = 0;
    for i in 0..k {
        ans += v[i] as u64;
    }
    println!("{}", ans);
}
