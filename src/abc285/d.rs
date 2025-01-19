use proconio::input;
use std::collections::HashMap;

fn dfs(g: &HashMap<String, String>, vis_all: &mut HashMap<String, bool>, vis: &mut HashMap<String, bool>, c: String) -> bool {
    match vis.get(&c) {
        Some(_v) => {
            return false
        },
        None => {
            match g.get(&c) {
                None => { return true },
                Some(ato) => {
                    vis.insert(c.to_string(), true);
                    vis_all.insert(c.to_string(), true);
        
                    return true && dfs(&g, vis_all, vis, ato.to_string());
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        g0: [(String, String); n],
    }

    let mut g: HashMap<String, String> = HashMap::new();
    let mut vis_all: HashMap<String, bool> = HashMap::new();
    for i in 0..n {
        g.insert(g0[i].0.to_string(), g0[i].1.to_string());
        vis_all.insert(g0[i].0.to_string(), false);
    }

    let mut res: bool = true;

    for i in 0..n {
        let mae: String = g0[i].0.to_string();

        let b: &bool = vis_all.get(&mae).unwrap();
        match b {
            true => {}, // 探索済み
            false => { // 未探索
                let mut vis: HashMap<String, bool> = HashMap::new();
                res = dfs(&g, &mut vis_all, &mut vis, mae);
            }
        }
        if !res { break; }
    }

    if res { println!("Yes") }
    else { println!("No") }
}
