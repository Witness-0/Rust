impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    if i > 0 && j > 0 {
                        let up = matrix[i - 1][j];
                        let left = matrix[i][j - 1];
                        let diag = matrix[i - 1][j - 1];
                        matrix[i][j] = 1 + up.min(left).min(diag);
                    }
                    ans += matrix[i][j];
                }
            }
        }
        ans
    }
}
