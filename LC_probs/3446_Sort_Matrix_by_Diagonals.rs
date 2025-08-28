use std::collections::HashMap;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        if m == 0 { return grid; }
        let n = grid[0].len();

        let mut buckets: HashMap<i32, Vec<i32>> = HashMap::new();
        for r in 0..m {
            for c in 0..n {
                buckets.entry(r as i32 - c as i32).or_default().push(grid[r][c]);
            }
        }

        for (&d, v) in buckets.iter_mut() {
            if d >= 0 {
                v.sort_unstable();
            } else {
                v.sort_unstable_by(|a, b| b.cmp(a));
            }
        }

        for r in 0..m {
            for c in 0..n {
                let d = r as i32 - c as i32;
                grid[r][c] = buckets.get_mut(&d).unwrap().pop().unwrap();
            }
        }

        grid
    }
}
