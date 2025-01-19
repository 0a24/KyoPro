use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    if l>n {
        println!("1");
    } else {
        let mut v: Vec<i32> = vec![0; n+1];
        for i in 0..l {
            v[i] = 1;        
        }
        for i in l..n+1 {
            v[i] = (v[i-l] + v[i-1]) % (10_i32.pow(9)+7);
        }   
        println!("{}", v[n]);    
    }
}
