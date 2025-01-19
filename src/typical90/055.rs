use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    }

    let mut ans: u64 = 0;
    let mut m: u64;
    
    for i1 in 0..n {
        for i2 in 0..i1 {
            for i3 in 0..i2 {
                for i4 in 0..i3 {
                    for i5 in 0..i4 {
                        m = a[i1] % p;
                        m = (m*a[i2]) % p;
                        m = (m*a[i3]) % p;
                        m = (m*a[i4]) % p;
                        m = (m*a[i5]) % p;
                        if m==q { ans += 1; }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
