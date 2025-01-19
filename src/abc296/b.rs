use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 8],
    }

    let mut ii: usize = 0;
    let mut jj: usize = 0;
    let r: Vec<char> = vec!['8', '7', '6', '5', '4', '3', '2', '1', ];
    let c: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', ];

    for i in 0..8 {
        for j in 0..8 {
            // print!("{:?}", s[i][j]);
            if s[i][j]=='*' {
                ii = i;
                jj = j;
            }
         }
        // println!("");
    }
    println!("{}{}", c[jj], r[ii]);
}
