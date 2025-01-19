use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }

    let mut res = vec![vec!['#'; c]; r];

    for i in 0..r {
        for j in 0..c {
            let v = b[i][j];
            match &v {
                '#' => (),
                '.' => res[i][j] = '.',
                _ => 
                    {
                        let p = v.to_digit(10).unwrap() as i32;
                        for k in 0..r {
                            for l in 0..c {
                                if ((i as i32 - k as i32).abs() + (j as i32 - l as i32).abs())<=p {
                                    res[k][l] = '.';
                                }
                            }
                        }
                    }
                }    
        }
    }

    for i in 0..r {
        let ans: String = res[i].iter().collect();
        println!("{}", ans);
    }

}
