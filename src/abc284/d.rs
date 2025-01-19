use proconio::input;

fn main() {
    input! {
        t: usize,
        ns: [u64; t],
    }
    
    for i in 0..t {
        let n: u64 = ns[i];

        let mut p: u64 = 1;
        let mut q: u64 = 1;

        if n%4==0 {
            p = 2;
            q = n/4;

        } else if n%2==0 {
            p = ((n/2) as f64).sqrt() as u64;
            q = 2;

        } else {
            let lg = (n as f64).sqrt() as u64 + 1;
            for j in 1..lg {
                let tmp: u64 = 2*j+1;
                if n%(tmp*tmp)==0 {
                    p = tmp;
                    q = n / (tmp*tmp);
                    break;
                } else if n%tmp==0 {
                    p = ((n / tmp) as f64).sqrt() as u64;
                    q = n / (p*p);
                    break;
                }
            }    
        }

        println!("{} {}", p, q);
    }
}
