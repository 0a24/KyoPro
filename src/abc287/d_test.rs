use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    
    let ls: usize = s.len();
    let lt: usize = t.len();
    
    for i in 0..lt+1 {
        let ss: Vec<char> = [&s[..i], &s[(ls-lt+i)..]].concat();
        // println!("{:?}\n", ss);

        let mut f: bool = true;
        for (&v, &w) in ss.iter().zip(t.iter()) {
            // println!("{} {}", v,w);
            // match v {
            //     w => (),
            //     '?' => (),
            //     _ => f = false,     
            // }

            if v==w { () }
            else if (v=='?') || (w=='?') { () }
            else { f = false; }

            if !f { break; }
        }

        println!("{}", ["No", "Yes"][f as usize]);
    }

}
