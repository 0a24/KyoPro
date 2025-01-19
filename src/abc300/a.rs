use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: [i32; n],
    }

    let d: i32 = a + b;
    for (i,&v) in c.iter().enumerate() {
        if v==d {
            println!("{}", i+1);
            break;
        }
    }
}
