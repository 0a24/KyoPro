use proconio::input;
// use proconio::marker::Chars;
use regex::Regex;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let re = Regex::new(&t).unwrap();

    let f = re.is_match(&s);

    if f { println!("{}", "Yes"); }
    else { println!("{}", "No"); }
}
