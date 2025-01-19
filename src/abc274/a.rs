use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let s = (b/a * 1000.0).round();
    let s_r = s / 1000.0;

    println!("{:.3}", s_r);
}
