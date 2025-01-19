use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut hmap: HashMap<(i32,i32), bool> = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    hmap.insert((x,y), true);

    let mut ans: bool = false;

    for c in s.iter() {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => (),
        }

        match hmap.get(&(x,y)) {
            None => {
                hmap.insert((x,y), true);
            } 
            Some(_) => {
                ans = true;
                break;
            }
        }
    }
    
    println!("{}", ["No", "Yes"][ans as usize]);
}