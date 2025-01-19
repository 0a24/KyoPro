use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    
    let words: Vec<String> = vec!["and".to_string(), "not".to_string(), "that".to_string(), "the".to_string(), "you".to_string()];

    let mut f: bool = false;
    for v in w.iter() {
        if words.contains(v) {
            f = true;
            break;
        }
    }
    println!("{}", ["No", "Yes"][f as usize]);
}
