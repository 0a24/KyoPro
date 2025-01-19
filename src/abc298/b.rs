use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[u8; n]; n],
        b: [[u8; n]; n],
    }

    let mut f: bool = true;
    for _ in 0..4 {
        // kaiten
        let mut tmp: Vec<Vec<u8>> = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                tmp[i][j] = a[i][j];
            }
        }
        for i in 1..n+1 {
            for j in 1..n+1 {
                a[i-1][j-1] = tmp[n+1-j-1][i-1];
            }
        }

        f = true;
        for i in 0..n {
            for j in 0..n {
                if (a[i][j] == 1) && (b[i][j] != 1) {
                    f = false;
                    break;
                } 
            }
            if !f { break }
        }
        if f { break }
    }
    
    println!("{}", ["No", "Yes"][f as usize]);
}
