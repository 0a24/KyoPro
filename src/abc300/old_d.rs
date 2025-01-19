use proconio::input;


fn main() {
    input! {
        n: i64,
    }

    let nn: i64 = (n as f64).sqrt() as i64 + 1;
    let mut ans = 0;

    for i in 1..nn {
        if n % i.pow(2) == 0 {
            let a = i;
            println!("a {}", a);

            let n_a = n / a.pow(2);
            for j in a+1..nn {
                if n_a % j.pow(2) == 0 {
                    print!("c {}, ", j);
                    let c = j;

                    let b = n_a / c.pow(2);
                    println!("b {}", b);

                    if a<b && b<c { ans += 1; }
                }
            }
            
        }
    }
    println!("{}", ans);

}
