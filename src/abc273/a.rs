use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        // s: [String; n],
    }

    let mut fv = 1;
    for k in 1..n+1 {
        fv = k * fv;
    }
    // let mut f = true;
    println!("{}", fv)

    // if f { println!("Yes") }
    // else { println!("No") }
}
