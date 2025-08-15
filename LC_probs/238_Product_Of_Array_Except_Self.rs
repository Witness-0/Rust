impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        // Initialize the answer vector with all elements = 1
        // We'll multiply values in place during prefix and suffix passes
        let mut ans = vec![1; n];

        /*
            Idea:
            - First pass (left to right): store prefix products
            - Second pass (right to left): multiply suffix products
            This avoids using division and extra space for prefix/suffix arrays.
        */

        // `low` will hold the running product of all elements to the left of index i
        let mut low = 1;
        for i in 0..n {
            ans[i] *= low;   // multiply with product of all elements before i
            low *= nums[i];  // update `low` by including nums[i]
        }

        // `high` will hold the running product of all elements to the right of index i
        let mut high = 1;
        for i in (0..n).rev() {
            ans[i] *= high;   // multiply with product of all elements after i
            high *= nums[i];  // update `high` by including nums[i]
        }

        ans
    }
}
