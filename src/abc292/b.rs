use proconio::input;
//use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    
    let mut v: Vec<u8> = vec![0; n];

    for _ in 0..q {
        input! {
            c: usize,
            x: usize,
        }
        
        match c {
            1 => v[x-1] += 1,
            2 => v[x-1] += 2,
            3 => {
                if v[x-1]>=2 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => (),
        }
    }
}
