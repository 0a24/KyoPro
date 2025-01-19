use proconio::input;
use proconio::marker::{Chars, Usize1};
use petgraph::unionfind::UnionFind;
use std::collections::HashMap;
use std::collections::HashSet;

// memo
// cat src/abc282/in.txt | ./target/debug/abc282_c
// cargo build --bin abc282_c

fn main() {
    println!("Hello, world!");

    input! {
        h: i32,
        _w: i32,
        mat: [Chars; h],
    }
    
    println!("{}", ["No", "Yes"][f as usize]);

    // String -> num
    let num: i32 = s.parse().unwrap();

    // String -> Vec<u32>
    const RADIX: u32 = 10;
    let num: Vec<u32> = x.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();


    // num -> char
    let c: char = std::char::from_digit(num as u32, 10).unwrap();

    // char -> num
    let num: i32 = c.to_digit(10).unwrap();

    // Vec init
    let vec = vec!["0".to_string(); n];
    
    // Vec<char> -> String
    let v_c = Vec<char>::new();
    let s: String = v_c.iter().collect();

    // Vec<usize> -> Vec<String>
    let s: Vec<String> = a.iter().map(|i: &usize| i.to_string()).collect();
    println!("{}", s.join(" "));

    // String -> Vec<char>
    let vc: Vec<char> = "abcd".chars().collect();
    
    // Vec<char> + Vec<char>
    a.extend(b.iter());

    // Vec sort_by descend
    v.sort_by(|a, b| (-a.1).partial_cmp(&(-b.1)).unwrap());
    
    // HashMap
    let mut hmap: HashMap<String, bool> = HashMap::new();
    match hmap.get(&c) {
        None => {
            f = false;
            break;
        } 
        Some(false) => {
            hmap.insert(c, true);
        }
        Some(true) => {
            f = false;
            break;
        }
    }

}

// num::integer::gcd(a, b);

// fn gcd(a: u64, b: u64) -> u64 {
//     let r: u64 = a % b;
//     if r == 0 { return b }
//     else { return gcd(b,r) }
// }


fn eratosthenes(n: usize)->Vec<u64> {
    let mut isprime: Vec<bool> = vec![true; n+1];
    isprime[0] = false; 
    isprime[1] = false;

    let mut np: usize = 0;

    for p in 2..n+1 {
        if !isprime[p] { continue; }

        np += 1;

        let mut q: usize = 2*p;
        while q <= n {
            isprime[q] = false;
            q += p;
        }
    }

    let mut ps: Vec<u64> = vec![0; np];
    let mut c: usize = 0;
    for i in 0..n+1 {
        if isprime[i] {
            ps[c] = i as u64;
            c += 1;
        }
    }

    return ps
}