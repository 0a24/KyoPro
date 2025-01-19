use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let mut n_u_max: (usize,usize) = (0,0);
    let mut n_u_max1: (usize,usize) = (0,0);
    for i in 0..n {
        if c[i]==t && r[i]>n_u_max.1 {
            n_u_max = (i+1,r[i]);
        }
        if c[i]==c[0] && r[i]>n_u_max1.1 {
            n_u_max1 = (i+1,r[i]);
        }
    }
    if n_u_max.0 > 0 {
        println!("{}", n_u_max.0);
    } else {
        println!("{}", n_u_max1.0);
    }
}
