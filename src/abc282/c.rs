use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
        // s: Bytes,
    }

    let mut s_out = Vec::new();
    // let mut s_out: Vec<Bytes> = Vec::new();

    let mut f = 0; 
    for &c in s.iter(){
        if c=='"' { f = (f+1)%2 }
        if f==0 && c==','{
            s_out.push('.');
        } else {
            s_out.push(c);
        }
    }

    let s_out2: String = s_out.iter().collect();
    // println!("{:?}", s_out);
    println!("{}", s_out2);
}
