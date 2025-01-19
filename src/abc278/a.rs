use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        // k: usize,
        a: [usize; n],
    }
    
    let s: Vec<String> = a.iter().map(|i: &usize| i.to_string()).collect();

    println!("{:?}", s);
    println!("{}", s.join(" "));
    
    
}


