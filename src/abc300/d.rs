use proconio::input;


fn eratosthenes(n: usize)->Vec<u64> {
    let mut isprime: Vec<bool> = vec![true; n+1];
    isprime[0] = false; 
    isprime[1] = false;

    let mut np: usize = 0;

    for p in 2..n+1 {
        if !isprime[p] { continue; }

        np += 1;

        let mut q: usize = 2*p;
        while q <= n {
            isprime[q] = false;
            q += p;
        }
    }

    let mut ps: Vec<u64> = vec![0; np];
    let mut c: usize = 0;
    for i in 0..n+1 {
        if isprime[i] {
            ps[c] = i as u64;
            c += 1;
        }
    }

    return ps
}

fn main() {
    input! {
        n: u64,
    }
    let mut ans: usize = 0;
    let ps: Vec<u64> = eratosthenes(10u64.pow(6) as usize);
    let lenps: usize = ps.len();
    
    for i in 0..lenps {
        let a: u64 = ps[i];
        if a*a*a*a*a >= n {
            // println!("{}", a);
            break;
        }

        for j in i+1..lenps {
            let b: u64 = ps[j];
            if a*a*b*b*b >= n { break; }

            for k in j+1..lenps {
                let c: u64 = ps[k];

                if a*a*b*c*c<=n { ans += 1; }
                
            }
        }
    }

    println!("{}", ans);
}
