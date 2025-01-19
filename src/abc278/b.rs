use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        mut h: i32,
        mut m: i32,
    }

    let mut mis_h: i32;
    let mut mis_m: i32;
    
    loop {
        // println!("{:02} {:02}", h, m);

        mis_h = (h/10)*10 + (m/10);
        mis_m = (h%10)*10 + (m%10);
        // println!("{:02} {:02} :mis", mis_h, mis_m);

        if mis_h<24 && mis_m<60 {
            break;
        }

        m += 1;
        if m==60 {
            m %= 60;
            h = (h+1) % 24;
        }
    }

    println!("{:02} {:02}", h, m);

}
