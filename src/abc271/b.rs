use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut ls: Vec<usize> = vec![0; n+1];
    // let mut a: Vec<i32> = vec![0; 2*100000];
    let mut a: Vec<i32> = vec![];
    for i in 0..n {
        input! {
            l: usize,
            tmp: [i32; l],
        }
        ls[i+1] = l + ls[i];
        for j in 0..l {
            // a[ls[i] + j] = tmp[j];
            a.push(tmp[j]);
        }
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }
        println!("{}", a[ls[s-1] + t-1]);
    }
}
