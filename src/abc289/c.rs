use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    let mut b: Vec<Vec<usize>> = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [usize; c],
        }
        b.push(a);
    }

    let mut res: u32 = 0;
    for i in 1..(2usize.pow(m)) {
        let mut v: Vec<bool> = vec![false; n];
        let mut j: usize = i;
        let mut idx_set: usize = 0;
        while j > 0 {
            if (j as u32 % 2) == 1 {
                for k in b[idx_set].iter() {
                    v[*k-1] = true;
                }
            }
            idx_set += 1;
            j >>= 1;
        }
        let mut f: bool = true;
        for h in v.iter() {
            f &= h;
        }
        if f {
            res += 1;
        }
    }
    println!("{}", res);
}