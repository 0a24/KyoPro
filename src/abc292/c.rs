use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut kake: Vec<u32> = vec![0; n+1];
    
    for i in 1..n+1 {
        let k = (i as f32).sqrt().floor() as usize;
        for j in 1..k+1 {
            if i%j==0 {
                kake[i] += 1;
                if j*j!=i {
                    kake[i] += 1;
                }
            }
        }
    }
    let mut res: u64 = 0;
    for j in 1..n {
        res += (kake[j]*kake[n-j]) as u64;
    }
    println!("{}", res);
}