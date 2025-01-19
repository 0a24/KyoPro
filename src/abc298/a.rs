use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut f: bool = false;

    for &v in s.iter() {
        match v {
            'x' => {
                f = false;
                break;
            },
            'o' => {
                f = true;
            }
            _ => (),
        }
    }

    println!("{}", ["No", "Yes"][f as usize]);
}
