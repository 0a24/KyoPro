use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [String; h],
    }

    let mut cnt: Vec<i32> = vec![0; w];
    let mut vc: Vec<char>;
    for j in 0..h {
        vc = c[j].chars().collect();
        for (i, &c) in vc.iter().enumerate() {
            if c=='#' {
                cnt[i] += 1;
            }
        }
    }

    let ans: Vec<String> = cnt.iter().map(|i| i.to_string()).collect();
    println!("{}", ans.join(" "));
}
