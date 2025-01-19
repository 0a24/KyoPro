use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let maxv: usize;
    match a.iter().max() {
        Some(n) => maxv = *n as usize,
        None => unreachable!(),
    }

    let mut cnt: Vec<i32> = vec![0; n];
    let mut chk_already: Vec<bool> = vec![false; maxv+1];
    
    for i in 0..n {
        for &j in a.iter() {
            // println!("{} {} {}", i, a[i], j);
            if j>a[i] && chk_already[j as usize]==false {
                cnt[i as usize] += 1;
                chk_already[j as usize] = true;
            }
            // println!("{:?}", cnt);
        }
        chk_already = vec![false; maxv+1];
    }
    println!("{:?}", maxv);
    println!("{:?}", cnt);
    println!("{:?}", chk_already);

    let mut cnt_out: i32;
    for k in 0..n {
        cnt_out = 0;
        for i in 0..n {
            if cnt[i]==k as i32 {
                cnt_out += 1;
            }
        }
        println!("{}", cnt_out);
    }

}
