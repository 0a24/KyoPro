use proconio::input;
// use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let v: Vec<char> = (0..26).map(|x| (x + b'A') as char).collect::<Vec<_>>();

    for i in 0..h {
        for j in 0..w {
            if a[i][j]==0 { print!("."); }
            else { print!("{}", v[a[i][j]-1]); }
        }
        println!();
    }
}
