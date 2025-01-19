use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    
    for _ in 0..t {
        let mut cnt: i32 = 0;
        input! {
            n: usize,
            a: [i32; n],
        }
        
        for j in 0..n {
            if a[j]%2 == 1 { cnt += 1; }
        }
        println!("{}", cnt);
    }    
}
