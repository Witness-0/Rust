// Sorting method
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.sort(); // sort in ascending order
        let mut missing = 1;

        for &num in &nums {
            if num > 0 && num == missing {
                missing += 1;
            }
        }

        missing
    }
}
