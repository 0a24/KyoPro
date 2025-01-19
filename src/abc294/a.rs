use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            a: u32,
        }
        if a%2==0 {
            print!("{} ", a);
        }
    }
    println!()
}
