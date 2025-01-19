use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let c: usize = s.len();
    let cs: Vec<char> = s.chars().collect();

    let mut i: usize = 0;
    let mut cnt: usize = 0;
    while i<(c-1) {
        if cs[i]=='0'&&cs[i+1]=='0' {
            cnt += 1;
            i += 1;
        }
        i += 1;
    }
    println!("{}", c-cnt);
}