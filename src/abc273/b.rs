use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        mut x: i64,
        k: i32,
    }

    let mut n: f64 = x as f64;
    let mut m: f64 = 1.0;
    for _ in 1..k+1 {
        m *= 10.0;
        n = (n / m).round() * m;
    }

    println!("{}", n)
}
