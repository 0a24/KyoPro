use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        x: i32,
        y: i32,
        z: i32,
    }

    if y*z>0 && y.abs()<z.abs() && x*y>0 && x.abs()>y.abs() {
        println!("{}", -1);
    } else if x*y<0 || (y*z>0 && z.abs()<y.abs()) || x.abs()<y.abs() {
        println!("{}", x.abs());
    } else {
        println!("{}", 2*z.abs()+x.abs());
    }
}
