use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    
    let ns: usize = s.len();
    let nt: usize = t.len();
    let mut pre: Vec<bool> = vec![false; nt+1];
    let mut suf: Vec<bool> = vec![false; nt+1];
    
    pre[0] = true;
    suf[0] = true;

    for i in 0..nt {
        if !pre[i] { break; }
        if (s[i]==t[i]) || (s[i]=='?') || (t[i]=='?') {
            pre[i+1] = true;
        }
    }

    for i in 0..nt {
        if !suf[i] { break; }
        if (s[ns-i-1]==t[nt-i-1]) || (s[ns-i-1]=='?') || (t[nt-i-1]=='?') {
            suf[i+1] = true;
        }
    }

    for i in 0..nt+1 {
        println!("{}", ["No", "Yes"][(pre[i]&&suf[nt-i]) as usize])
    }
    
}
