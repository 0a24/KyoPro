use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    
    if b==2*a || b==2*a+1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
