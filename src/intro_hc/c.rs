use proconio::input;

fn b() {
    ()
}

fn main() {
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
    }

    input! {
        mut t: [usize; d], // テストのタイプ

        m: usize, // 変更回数
        d_q: [(i32, i32); m], // 日, タイプ
    }


    //
    t[d_q[0].0 as usize - 1] = d_q[0].1 as usize;
    //
    

    let mut lastdays: Vec<i32> = vec![0; 26];
    let mut score: i32 = 0;
    for i in 1..d+1 { // i日目
        lastdays[t[i-1]-1] = i as i32;

        let mut loss: i32 = 0;
        for j in 1..26+1 { // テストのタイプ
            loss += c[j-1] * (i as i32 - lastdays[j-1]);
        }
        score += s[i-1][t[i-1]-1] - loss;
    }
    println!("{}", score);


}
