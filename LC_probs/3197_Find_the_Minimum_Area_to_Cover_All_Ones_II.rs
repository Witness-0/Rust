impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = (m * n) as i32;

        fn m_a(g: &Vec<Vec<i32>>, si: usize, ei: usize, sj: usize, ej: usize) -> i32 {
            if si > ei || sj > ej { return 0; }
            let mut found = false;
            let mut min_i = usize::MAX;
            let mut min_j = usize::MAX;
            let mut max_i = 0usize;
            let mut max_j = 0usize;

            for i in si..=ei {
                for j in sj..=ej {
                    if g[i][j] == 1 {
                        found = true;
                        if i < min_i { min_i = i; }
                        if j < min_j { min_j = j; }
                        if i > max_i { max_i = i; }
                        if j > max_j { max_j = j; }
                    }
                }
            }
            if !found { 0 } else { ((max_i - min_i + 1) * (max_j - min_j + 1)) as i32 }
        }

        for i in 0..m {
            let top = m_a(&grid, 0, i, 0, n - 1);
            for j in 0..n {
                let a = m_a(&grid, i + 1, m.saturating_sub(1), 0, j);
                let b = m_a(&grid, i + 1, m.saturating_sub(1), j + 1, n.saturating_sub(1));
                ans = ans.min(top + a + b);
            }
        }

        for i in 0..m {
            let bottom = m_a(&grid, i, m - 1, 0, n - 1);
            for j in 0..n {
                let a = m_a(&grid, 0, i.saturating_sub(1), 0, j);
                let b = m_a(&grid, 0, i.saturating_sub(1), j + 1, n.saturating_sub(1));
                ans = ans.min(bottom + a + b);
            }
        }

        for j in 0..n {
            let left = m_a(&grid, 0, m - 1, 0, j);
            for i in 0..m {
                let a = m_a(&grid, 0, i, j + 1, n.saturating_sub(1));
                let b = m_a(&grid, i + 1, m.saturating_sub(1), j + 1, n.saturating_sub(1));
                ans = ans.min(left + a + b);
            }
        }

        for j in 0..n {
            let right = m_a(&grid, 0, m - 1, j, n - 1);
            for i in 0..m {
                let a = m_a(&grid, 0, i, 0, j.saturating_sub(1));
                let b = m_a(&grid, i + 1, m.saturating_sub(1), 0, j.saturating_sub(1));
                ans = ans.min(right + a + b);
            }
        }

        for i1 in 0..m {
            for i2 in i1 + 1..m {
                let a = m_a(&grid, 0, i1, 0, n - 1);
                let b = m_a(&grid, i1 + 1, i2, 0, n - 1);
                let c = m_a(&grid, i2 + 1, m - 1, 0, n - 1);
                ans = ans.min(a + b + c);
            }
        }

        for j1 in 0..n {
            for j2 in j1 + 1..n {
                let a = m_a(&grid, 0, m - 1, 0, j1);
                let b = m_a(&grid, 0, m - 1, j1 + 1, j2);
                let c = m_a(&grid, 0, m - 1, j2 + 1, n - 1);
                ans = ans.min(a + b + c);
            }
        }

        ans
    }
}
