use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    
    let mut vis: Vec<bool> = vec![false; w];

    for k in 0..w {
        // println!("\n{}", k);
        for j in 0..w {
            if vis[j] {
                ()
            } else {
                let mut b: bool = true;
                for i in 0..h {
                    // println!("{} {}", s[i][k], t[i][j]);
                    if s[i][k]!=t[i][j] {
                        b = false;
                        break;
                    }
                }
                if b { vis[j] = true; }
            }
        }
    }
    
    for v in vis.iter() {
        if !v {
            println!("No");
            return;
        }
    }
    println!("Yes");
    return;
}
