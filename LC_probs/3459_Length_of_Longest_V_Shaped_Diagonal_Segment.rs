impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut mem = vec![vec![vec![vec![vec![-1i32; 4]; 2]; 2]; n]; m];

        const DIRS: [(i32, i32); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

        fn dfs(i: i32, j: i32, turned: usize, need: i32, dir: usize, g: &Vec<Vec<i32>>, mem: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>) -> i32 {
            let m = g.len() as i32;
            let n = g[0].len() as i32;
            if i < 0 || i >= m || j < 0 || j >= n {
                return 0;
            }
            let ui = i as usize;
            let uj = j as usize;

            if g[ui][uj] != need {
                return 0;
            }

            let hash_num = if need == 2 { 1 } else { 0 };
            if mem[ui][uj][turned][hash_num][dir] != -1 {
                return mem[ui][uj][turned][hash_num][dir];
            }

            let (dx, dy) = DIRS[dir];
            let next_need = if need == 2 { 0 } else { 2 };
            let mut best = 1 + dfs(i + dx, j + dy, turned, next_need, dir, g, mem);

            if turned == 0 {
                let next_dir = (dir + 1) % 4;
                let (ndx, ndy) = DIRS[next_dir];
                best = best.max(1 + dfs(i + ndx, j + ndy, 1, next_need, next_dir, g, mem));
            }

            mem[ui][uj][turned][hash_num][dir] = best;
            best
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for d in 0..4 {
                        let (dx, dy) = DIRS[d];
                        let ni = i as i32 + dx;
                        let nj = j as i32 + dy;
                        ans = ans.max(1 + dfs(ni, nj, 0, 2, d, &grid, &mut mem));
                    }
                }
            }
        }
        ans
    }
}
