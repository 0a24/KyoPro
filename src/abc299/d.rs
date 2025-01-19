use proconio::input;
use std::io::{stdout, Write};

// static S: &str = "000111100110011";
// fn oracle(qu: usize)->u8 {
//     let ss: Vec<char> = S.chars().collect();
//     let s: u8 = ss[qu as usize -1].to_digit(10).unwrap() as u8;
//     return s
// }

fn main() {
    input! {
        n: usize,
    }
    // let n: usize = S.len();
    let n_loop: usize = 20;

    let mut i_zero: usize = 1;
    let mut i_one: usize = n;

    for _i in 0..n_loop {
        let qu: usize = (i_zero + i_one) / 2;
        println!("? {}", qu);
        stdout().flush().unwrap();

        input! {
            s: u8,
        }
        // let s: u8 = oracle(qu);
        // println!("ans {}", s);

        match s {
            0 => {
                i_zero = qu;
            },
            1 => {
                i_one = qu;
            },
            _ => (),
        }

        if i_one-i_zero == 1 {
            break
        }
    }
    println!("! {}", i_zero);
    stdout().flush().unwrap();
    
    // let res0 = oracle(i_zero);
    // let res1 = oracle(i_one);
    // println!("{} {}", res0, res1);
}
