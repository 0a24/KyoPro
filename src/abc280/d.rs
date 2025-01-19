use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        mut k: u64,
    }
    let k0: u64 = k;
    let k2 = (k as f64).sqrt() as u64 + 1;
    let mut ps: Vec<u64> = vec![];
    for i in 2..k2 {
        while k%i==0 {
            ps.push(i);
            k /= i;
        }
        if k==1 { break; }
    }
    
    if ps.len()==0 {
        println!("{}", k0);
        return;
    }
    
    let mut mp: u64 = ps.pop().unwrap();
    let mut l: u64 = 1;
    for i in 1..mp+1 {
        l *= i;
    }
    while l<k0 {
        mp += 1;
        l *= mp;
    }
    println!("{}", mp);
}
