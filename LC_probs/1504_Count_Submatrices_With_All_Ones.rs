impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        if n == 0 { return 0; }
        let m = mat[0].len();

        // heights[j] = consecutive 1s ending at current row in column j
        let mut heights = vec![0usize; m];
        let mut ans: i64 = 0;

        for i in 0..n {
            // update histogram heights for this row
            for j in 0..m {
                if mat[i][j] == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }

            // Monotonic non-decreasing stack of indices by heights
            // count[k] = how many consecutive positions this bar represents
            let mut stack: Vec<usize> = Vec::with_capacity(m);
            let mut count = vec![0usize; m];

            // sum = total number of all-ones submatrices whose bottom row is i,
            // considering columns processed so far.
            let mut sum: i64 = 0;

            for j in 0..m {
                let mut cnt_here: usize = 1;

                while let Some(&top) = stack.last() {
                    if heights[top] >= heights[j] {
                        stack.pop();
                        let c = count[top];
                        cnt_here += c;
                        // remove contribution of the popped bar
                        sum -= (heights[top] as i64) * (c as i64);
                    } else {
                        break;
                    }
                }

                stack.push(j);
                count[j] = cnt_here;

                // add contribution of current bar across its width
                sum += (heights[j] as i64) * (cnt_here as i64);

                // every step, sum is number of submatrices ending at column j with bottom at row i
                ans += sum;
            }
        }

        ans as i32
    }
}
