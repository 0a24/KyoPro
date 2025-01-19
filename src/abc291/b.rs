use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [i32; 5*n],
    }

    x.sort();
    let mut res: f32 = 0.0;
    for i in n..4*n {
        res += x[i] as f32;
    }
    
    println!("{}", res/(3.0*n as f32));
}
