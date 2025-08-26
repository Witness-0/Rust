impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_dsq = 0; 
        let mut max_a = 0;

        for rect in dimensions {
            let l = rect[0];
            let w = rect[1];
            let diag_sq = l * l + w * w;
            let area = l * w;

            if diag_sq > max_dsq {
                max_dsq = diag_sq;
                max_a = area;
            } else if diag_sq == max_dsq {
                max_a = max_a.max(area);
            }
        }

        max_a
    }
}
