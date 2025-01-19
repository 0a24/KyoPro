use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i32; n],
    }
    
    let mut t2: usize = 0;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        }

        let xx: usize = (n+x-1-t2) % n;
        let yy: usize = (n+y-1-t2) % n;

        match t {
            1 => a.swap(xx, yy),
            2 => t2 += 1,
            3 => println!("{}", a[xx]),
            _ => (),
        }
    }
}
