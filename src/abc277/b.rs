use proconio::input;
// use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    
    let mut map: HashMap<String, bool> = HashMap::new();
    let suits: Vec<char> = vec!['H', 'D', 'C', 'S'];
    let nums: Vec<char> = "A23456789TJQK".chars().collect();
    let mut c_str: String;

    for &su in suits.iter() {
        for &n in nums.iter() {
            c_str = vec![su, n].iter().collect();
            map.insert(c_str, false);
        }    
    }

    let mut f: bool = true;
    for card in s{
        match map.get(&card) {
            None => {
                f = false;
                break;
            } 
            Some(false) => {
                map.insert(card, true);
            }
            Some(true) => {
                f = false;
                break;
            }
        }
    }

    if f { println!("Yes") }
    else { println!("No") }

}
