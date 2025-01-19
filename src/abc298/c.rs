
use proconio::input;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut btmap: Vec<BTreeMap<u32, u32>> = vec![BTreeMap::new(); n];
    let mut btmap_hako: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 2*(10u32.pow(5) as usize)];

    for _ in 0..q {
        input! {
            qu: u32,
        }
        // println!("{:?}", qu);
        match qu {
            1 => {
                input! { i: u32, j: usize }
                match btmap[j-1].get(&i) {
                    Some(&v) => {
                        btmap[j-1].insert(i, v+1);
                    },
                    _ => {
                        btmap[j-1].insert(i, 1);
                    }
                }

                btmap_hako[i as usize -1].insert(j);

            },
            2 => {
                input! { i: usize }
                for (&k,&v) in btmap[i-1].iter() {
                    for _ in 0..v {
                        print!("{} ", k);
                    }
                }
                println!();
            },
            3 => {
                input! { i: usize }
                for h in btmap_hako[i-1].iter() {
                    print!("{} ", h)
                }
                println!();
            },
            _ => (),
        }

        // println!("{:?}", btmap);
    }
}