impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // we use the concept of valid region and unvisited portion
        // [region for 0 | unk region | region for 2]
        // so when we come across "zero" in our unk region, swap it with the number in zero's 
        // valid region, similarly for "twos".

        if nums.is_empty() {
            return; // Edge case: no elements to sort
        }

        let mut low: usize = 0;
        let mut mid: usize = 0;
        let mut high: usize = nums.len() - 1;

        // high is n-1, since its empty initially, similarly mid = 0.
        // we run a while loop until the unk region crosses our 2's region.
        while mid <= high {
            if nums[mid] == 0 {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            }
            else if nums[mid] == 2 {
                nums.swap(high, mid);
                if high == 0 { break; } // prevent usize underflow
                high -= 1;
            }
            else {
                mid += 1;
            }
        }
    }
}
