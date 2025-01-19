use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    let r: u64 = a % b;
    if r == 0 { return b }
    else { return gcd(b,r) }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    
    let mut p2: Vec<u64> = vec![0; 33];
    let mut p3: Vec<u64> = vec![0; 33];
    p2[0] = 1;
    p3[0] = 1;
    for i in 1..33 {
        p2[i] = p2[i-1]*2;
        p3[i] = p3[i-1]*3;
    }

    let mut d: u64 = gcd(a[0], a[1]);
    for i in 2..n {
        d = gcd(d, a[i]);
    }

    let mut n2: Vec<u32> = vec![0; n];
    let mut n3: Vec<u32> = vec![0; n];
    for i in 0..n {
        for j in 0..33 {
            if gcd(a[i], p2[j])==p2[j] {
                n2[i] = j as u32;
            }
            if gcd(a[i], p3[j])==p3[j] {
                n3[i] = j as u32;
            }
        }
    }

    let n2min = *n2.iter().min().unwrap();
    let n3min = *n3.iter().min().unwrap();

    let mut ans: u64 = 0;
    let mut f: bool = true;
    for i in 0..n {
        if a[i] / (2u64.pow(n2[i] - n2min)) / (3u64.pow(n3[i] - n3min)) == d {
            ans += (n2[i] + n3[i]) as u64;
        } else {
            f = false;
            break;
        }
    }

    if !f { println!("-1"); }
    else {
        println!("{}", ans - n as u64 * (n2min + n3min) as u64);
    }
}
