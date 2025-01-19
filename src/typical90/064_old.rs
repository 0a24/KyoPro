use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        // lrv: [(i32,i32,i32); q],
    }

    let mut ans: i64;
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            v: i64,
        }
        for j in l-1..r {
            a[j] += v;
        }
        // println!("{:?}", a);
        ans = 0;
        for k in 0..n-1 {
            ans += (a[k]-a[k+1]).abs();
        }
        println!("{}", ans);
    }
}
