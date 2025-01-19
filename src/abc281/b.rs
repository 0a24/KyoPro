use proconio::input;
// use proconio::marker::Chars;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"^[A-Z][1-9][0-9]{5}[A-Z]$").unwrap();
    let f = re.is_match(&s);

    if f { println!("{}", "Yes"); }
    else { println!("{}", "No"); }

}
