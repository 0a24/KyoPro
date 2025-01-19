use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    
    let mut cnt: u32 = 0;

    for v in s.iter() {
        if *v == "For".to_string() {
            cnt += 1;
        }
    }

    if cnt > (n as u32/2) {
        println!("Yes")
    } else {
        println!("No")
    }
}
