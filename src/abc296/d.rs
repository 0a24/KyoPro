use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let mut x: u64 = std::u64::MAX;
    for a in 1..n+1{
        let b: u64 = (m as f64 / a as f64).ceil() as u64;
        if b > n { continue; }
        if a > b { break; }
        
        x = std::cmp::min(x, a * b);
    }
    
    if x==std::u64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", x);
    }
}
