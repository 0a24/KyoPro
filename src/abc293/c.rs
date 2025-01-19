
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[Usize1; w]; h],
    }

    let mut ans: usize = 0;
    let mov: usize = (h-1)+(w-1);
    for rt0 in 0..2_u32.pow(mov as u32) as usize {
        let mut rt: usize = rt0;

        let mut i_s: usize = 0;
        let mut j_s: usize = 0;
        for _ in 0..mov {
            if rt%2==0 { i_s += 1 }
            else { j_s += 1 }
            rt >>= 1;
        }
        if !((i_s==h-1)&(j_s==w-1)) { () }
        else {
            let mut hmap: HashMap<usize, bool> = HashMap::new();
            let mut f: bool = true;

            let mut rt: usize = rt0;
            let mut i = 0;
            let mut j = 0;
            hmap.insert(a[0][0], true);
            for _ in 0..mov {
                if rt%2==0 { i += 1 }
                else { j += 1 }

                match hmap.get(&a[i][j]) {
                    None => {
                        hmap.insert(a[i][j], true);
                    } 
                    Some(_) => {
                        f = false;
                        break;
                    }
                }
                rt >>= 1;
            }
            if f&(i==h-1)&(j==w-1) {
                ans += 1;
            }
        }        
    }

    println!("{}", ans);
}