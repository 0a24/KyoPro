use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        t: [i32; n],
    }

    let mut f: i32 = -1;

    for i in 0..n-1 {
        if t[i+1]-t[i]<=d {
            f = t[i+1];
            break;
        }
    }

    println!("{}", f);
}
