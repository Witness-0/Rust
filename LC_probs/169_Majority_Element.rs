impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 { //made the vector mutable for sorting.
        nums.sort();
        let low : usize = 0;
        let high : usize = nums.len() - 1;
        let mid : usize = (low + high) >> 1;

        //since there's always a majo element [present n/2 times], after sorting it will pass mid index in any case.  
        nums[mid] as i32
    }
}
