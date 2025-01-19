use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }
    
    let mut cnt: u32 = 0;

    let mut l: Vec<bool> = vec![false; 1000];
    for v in t.iter() {
        l[*v] = true;
    }

    for v in s.iter() {
        if l[v % 1000] == true {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
