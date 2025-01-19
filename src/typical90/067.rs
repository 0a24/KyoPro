use proconio::input;

fn main() {
    input! {
        mut n8: u128,
        k: usize,
    }

    let mut n10: u128;
    let mut b: u128;
    let mut tmp: u128;

    for _ in 0..k {
        b = 1;
        n10 = 0;
        while n8>0 {
            n10 += (n8%10) * b;
            b *= 8;
            n8 /= 10;
        }

        b = 1;
        n8 = 0;
        while n10>0 {
            tmp = n10 % 9;
            if tmp == 8 { tmp = 5; }
            n8 += tmp * b;
            b *= 10;
            n10 /= 9;
        }
    }    
    println!("{}", n8);
}
