use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
    }

    let mut a2: usize;
    let mut b2: usize;
    let mut c2: usize;
    let mut na: Vec<i64> = vec![0; 46];
    let mut nb: Vec<i64> = vec![0; 46];
    let mut nc: Vec<i64> = vec![0; 46];
    for i in 0..n {
        a2 = a[i] as usize % 46;
        b2 = b[i] as usize % 46;
        c2 = c[i] as usize % 46;
        na[a2] += 1;
        nb[b2] += 1;
        nc[c2] += 1;
    }
    
    let mut cnt: i64 = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i+j+k)%46 == 0 {
                    cnt += na[i]*nb[j]*nc[k];
                }
            }
        }
    }
    println!("{}", cnt);
}
