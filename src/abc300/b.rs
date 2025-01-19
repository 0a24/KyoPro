use proconio::input;
use proconio::marker::Chars;

fn check_mat(h: usize, w: usize, a: &Vec<Vec<char>>, b: &Vec<Vec<char>>)->bool {
    let mut f: bool = true;
    for i in 0..h {
        for j in 0..w {
            if a[i][j]!=b[i][j] {
                f = false;
            }
            if !f { break; }
        }
        if !f { break; }
    }
    return f;
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h],
        b: [Chars; h],
    }

    let mut f: bool = false;
    for _ in 0..h+1 {
        // 縦のシフト
        let mut tmp_w: Vec<char> = vec![' '; w];
        for i in 0..w {
            tmp_w[i] = a[0][i];
        }
        for i in 0..h-1 {
            for j in 0..w {
                a[i][j] = a[i+1][j];
            }
        }
        for i in 0..w {
            a[h-1][i] = tmp_w[i];
        }

        for _ in 0..w+1 {
            // 横のシフト
            let mut tmp_h: Vec<char> = vec![' '; h];
            for i in 0..h {
                tmp_h[i] = a[i][0];
            }
            for i in 0..w-1 {
                for j in 0..h {
                    a[j][i] = a[j][i+1];
                }
            }
            for i in 0..h {
                a[i][w-1] = tmp_h[i];
            }            

            f = check_mat(h, w, &a, &b);
            if f { break; }
        }
        if f { break; }
    }

    println!("{}", ["No", "Yes"][f as usize]);
}
