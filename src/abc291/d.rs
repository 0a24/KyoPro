use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [[u64; 2]; n],
    }
    let m: u64 = 998244353;
    let mut n_omote_ura: (u64, u64) = (0, 0);
    let mut n_pre: (u64, u64) = (1, 1);

    for i in 1..n {
        n_omote_ura = (0, 0);

        if ab[i-1][0] != ab[i][0] { n_omote_ura.0 += n_pre.0; }
        if ab[i-1][1] != ab[i][0] { n_omote_ura.0 += n_pre.1; }
        n_omote_ura.0 %= m;

        if ab[i-1][0] != ab[i][1] { n_omote_ura.1 += n_pre.0; }
        if ab[i-1][1] != ab[i][1] { n_omote_ura.1 += n_pre.1; }
        n_omote_ura.1 %= m;

        n_pre = n_omote_ura;
    }
    if n>1 {
        println!("{}", (n_omote_ura.0 + n_omote_ura.1)%m);
    } else {
        println!("2");
    }
}
