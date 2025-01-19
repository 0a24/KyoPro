
use proconio::input;
// use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut users: HashMap<String, usize> = HashMap::new();
    for (i, k) in s.iter().enumerate() {
        match users.get(k) {
            Some(_) => (),
            None => { 
                println!("{}", i+1);
                users.insert(k.to_string(),i);
             },
        }   
    }
}
