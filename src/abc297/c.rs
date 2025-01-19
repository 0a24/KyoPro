
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut res: Vec<Vec<char>> = vec![vec!['.'; w]; h];

    for i in 0..h {
        res[i][w-1] = s[i][w-1];

        let mut f: bool = true;

        for j in 0..w-1 {
            if s[i][j]=='T' && s[i][j+1]=='T' && f {
                res[i][j] = 'P';
                res[i][j+1] = 'C';

                f = false;

            } else if s[i][j]=='T' && f {
                res[i][j] = 'T';
                f = true;
            } else {
                f = true;
            }
        }
    }

    for i in 0..h {
        let out: String = res[i].iter().collect();
        println!("{}", out);
    }
}