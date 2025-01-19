use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    let mut s = 0;
    for &i in a.iter() {
        s += i;
    }
    println!("{}", s)
}
