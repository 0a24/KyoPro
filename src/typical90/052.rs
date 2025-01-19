use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [[i32; 6]; n],
    }

    let m: i64 = 10_i64.pow(9) + 7;
    let mut ans: i64 = 1;
    
    for v in a.iter() {
        ans *= v.iter().sum::<i32>() as i64 % m;
        ans %= m;
    }

    println!("{}", ans);
}
