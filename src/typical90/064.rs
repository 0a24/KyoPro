use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
    }

    let mut d: Vec<i64> = vec![0; n-1];
    let mut ans: i64 = 0;
    for i in 0..n-1 {
        d[i] = a[i+1]-a[i];
        ans += d[i].abs();
    }

    let mut tmp: i64;
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            v: i64,
        }

        if l!=1 { 
            tmp = d[l-2] + v;
            ans += tmp.abs() - d[l-2].abs();
            d[l-2] = tmp;
        }

        if r!=n { 
            tmp = d[r-1] - v;
            ans += tmp.abs() - d[r-1].abs();
            d[r-1] = tmp;
        }
        
        println!("{}", ans);
    }
}
