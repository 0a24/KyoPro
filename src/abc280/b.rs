use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [i32; n],
    }

    let mut s_out = vec![0; n];
    let mut s_all: i32 = 0;
    s_out[0] = s[0];
    
    for i in 1..n {
        s_all += s_out[i-1];
        s_out[i] = s[i] - s_all;
    }

    let st_out: Vec<String> = s_out.iter().map(|i| i.to_string()).collect();
    println!("{}", st_out.join(" "));
    
}
