use proconio::input;

fn main() {
    input! {n: u32, k: usize};
    input! {mut v: [String; n]};

    for i in &v[0..k] {
        println!("{}", i);
    }

    v[0..k].sort();

    (0..k).for_each(|i| {
        println!("{}", v[i]);
    });
}