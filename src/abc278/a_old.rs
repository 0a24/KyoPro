use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [String; n],
    }
    
    let mut vec: Vec<String> = Vec::<String>::new();

    if n <= k {
        vec = vec!["0".to_string(); n];
        
    } else {
        vec = a[k..].to_vec();

        for _ in 0..k {
            vec.push("0".to_string());
        }

    }
    // add space
    let mut s: String = vec[0].to_string();

    for v in vec[1..].iter(){
        s.push_str(" ");
        s.push_str(v);
    }
    println!("{}", s);
    
    
}


