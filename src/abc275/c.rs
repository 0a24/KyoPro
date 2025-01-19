use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 9],
    }
    let mut ans: u32 = 0;
    
    for dx in 1..9 {
        for dy in 0..9 {
            for r in 0..9 {
                for c in 0..9 {
                    let mut f: bool = true;
                    if 0<=(r+dy) && (r+dy)<=8 && 
                       0<=(c+dx) && (c+dx)<=8 && 
                       0<=(r+dx) && (r+dx)<=8 && 
                       0<=(c-dy as i8) && (c-dy as i8)<=8 && 
                       0<=(r+dx+dy) && (r+dx+dy)<=8 &&
                       0<=(c+dx-dy as i8) && (c+dx-dy as i8)<=8 {
                        f &= s[r as usize][c as usize]=='#';
                        f &= s[(r+dy) as usize][(c+dx) as usize]=='#';
                        f &= s[(r+dx) as usize][(c-dy) as usize]=='#';
                        f &= s[(r+dx+dy) as usize][(c+dx-dy) as usize]=='#';
                        // if r==0 && c==0 && dy==0 && dx==1 {
                        //     println!("{}", s[r as usize][c as usize]);
                        //     println!("{}", s[(r+dy) as usize][(c+dx) as usize]);
                        //     println!("{}", s[(r+dx) as usize][(c-dy) as usize]);
                        //     println!("{}", s[(r+dx+dy) as usize][(c+dx-dy) as usize]);
                        //     println!("{}", f);
                        // }
                    } else {
                        f = false;
                        // println!("{} {} {} {}", r, c, dx, dy);
                    }
                    if f { ans += 1; }
                }
            }
        }
    }
    println!("{}", ans);
}
