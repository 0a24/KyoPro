
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        mut b: [u32; m],
    }

    let mut ia: usize = 0;
    let mut ib: usize = 0;

    let mut va: Vec<usize> = vec![];
    let mut vb: Vec<usize> = vec![];

    let mut f_ab: u32 = 0;

    for i in 0..(n+m) {
        if a[ia]<b[ib] {
            ia += 1;
            va.push(i+1);
        } else {
            ib += 1;
            vb.push(i+1);
        }
        if ia==n {
            f_ab = 1;
            break;
        } else if ib==m {
            f_ab = 2;
            break;
        }
    }

    let mut c: usize = va.len() + vb.len() + 1;
    if f_ab==2 {
        for _ in 0..n-va.len() {
            va.push(c);
            c += 1;
        }
    }
    if f_ab==1 {
        for _ in 0..m-vb.len() {
            vb.push(c);
            c += 1;
        }
    }

    for i in 0..n {
        print!("{} ", va[i]);
    }
    println!();
    for i in 0..m {
        print!("{} ", vb[i]);
    }
    println!();

}