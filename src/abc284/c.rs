use proconio::input;

static mut G: Vec<Vec<usize>> = Vec::new();
static mut VIS: Vec<bool> = Vec::new();

fn dfs(c: usize) {
    unsafe {
        VIS[c-1] = true;
        for v in G[c-1].iter() {
            if !VIS[v-1] {
                dfs(*v);
            }
        }
    }    
}


fn main() {
    input! {
        n: usize,
        m: usize,
    }

    unsafe {
        G = vec![vec![]; n];
        VIS = vec![false; n];

        for _ in 0..m {
            input! {
                u: usize,
                v: usize,
            }
            G[u-1].push(v);
            G[v-1].push(u);
        }
        // println!("{:?}", G);

        let mut ans: u32 = 0;
        for i in 1..n+1 {
            if !VIS[i-1] {
                ans += 1;
                dfs(i);
            }
        }
        println!("{}", ans);
    }
}
