
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        x: i32,
        mut a: [i32; n],
    }

    let mut hset: HashSet<i32> = HashSet::new();
    for v in a.iter() {
        hset.insert(*v);
    }

    let mut f: bool = false;

    for v in a.iter() {
        f = hset.contains(&(v+x));
        if f { break; }
    }

    println!("{}", ["No", "Yes"][f as usize]);
}