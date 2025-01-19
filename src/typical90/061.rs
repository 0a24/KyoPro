use proconio::input;
// use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(u32, u32); q],
    }

    let mut dq: VecDeque<u32> = VecDeque::new();

    for (t, x) in tx.iter() {
        match t {
            1 => { dq.push_front(*x); }
            2 => { dq.push_back(*x); }
            3 => { println!("{}", dq[*x as usize -1]); }
            _ => ()
        }
    }
}
