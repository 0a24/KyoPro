use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    }
    
    if a==b {
        println!("{}", 0);
        return
    }

    let mut c: u64;

    if a < b {
        c = a;
        a = b;
        b = c;
    }

    let mut ans: u64 = 0;

    while b != 0 {
        ans += a / b;
        a = a % b;
        c = a;
        a = b;
        b = c;
        // println!("{} {} {}", a, b, ans);
    }

    println!("{}", ans-1);
}
