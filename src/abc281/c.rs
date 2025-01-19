use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        t: i64,
        a: [i64; n],
    }

    let sum: i64 = a.iter().sum();
    let mut r = t % sum;

    let mut idx = 1;
    for tmp in a.iter() {
        if r-tmp < 0 {
            break;
        }
        r -= tmp;
        idx += 1;
    }

    println!("{} {}", idx, r)

}
