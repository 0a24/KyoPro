use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let u: u32 = a.pow(b);
    println!("{}", u);
}
