use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        _s: usize,
        mut a: [u32; n],
    }
    
    let mut tmp: u32;
    let t: usize = q - p + 1;

    for i in 0..t {
        tmp = a[p-1 + i];
        a[p-1 + i] = a[r-1 + i];
        a[r-1 + i] = tmp;
    }

    for c in a.iter() {
        print!("{} ", c);
    }
    println!();
}
