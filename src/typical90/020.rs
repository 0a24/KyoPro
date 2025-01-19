use proconio::input;
// use proconio::marker::Chars;

// fn blen(v: i64) -> i64 {
//     format!("{:b}", v).to_string().len() as i64
// }

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let mut c2b = 1;
    for _ in 0..b {
        c2b *= c;
    }
    
    if a < c2b {
        println!("Yes");
    } else {
        println!("No")
    }
}
