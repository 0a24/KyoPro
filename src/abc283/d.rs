use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }

    let sv: Vec<char> = s.chars().collect();
    let mut map: HashMap<char, (bool,u32)> = HashMap::new();
    let mut r: u32 = 0;
    
    for i in 0..sv.len() {
        if sv[i]=='(' {
            r += 1;
            
        } else if sv[i]==')' {
            r -= 1;
            // change false
            let mut tmp: Vec<char> = vec![];
            for v in map.iter() {
                if (v.1).0==true && (v.1).1==r+1 {
                    tmp.push(*v.0);
                }
            }
            for v in tmp {
                map.insert(v, (false,r));
            }

        } else {
            match map.get(&sv[i]) {
                None => {
                    map.insert(sv[i], (true,r));
                } 
                Some(v) => {
                    if v.0==true {
                        println!("No");
                        return;
                    } else {
                        map.insert(sv[i], (true,r));
                    }
                }
            }
        }
    }

    println!("Yes");
}