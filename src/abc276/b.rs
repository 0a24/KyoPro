use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [[i32; 2]; m],
    }
    
    let mut table: Vec<Vec<i32>> = vec![vec![]; n+1];
    for i in 0..m {
        table[ab[i][0] as usize].push(ab[i][1]);
        table[ab[i][1] as usize].push(ab[i][0]);
    }
    
    let mut vs: Vec<String>;
    let mut s: String;
    for i in 1..table.len() {
        table[i].sort();
        vs = table[i].iter().map(|i: &i32| i.to_string()).collect();
        s = vs.join(" ");

        println!("{} {}", table[i].len(), s);
    }
}
