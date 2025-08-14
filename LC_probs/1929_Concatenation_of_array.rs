impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len(); // to store the size of input array.
        let mut ans = vec![0;2*n]; // we declared a vector of size 2*n, with all elements as 0.
        for i in 0..n { // loop from 0 to n-1.
            ans[i] = nums[i]; //copying out the element in first half
            ans[i + n] = nums[i]; //second half
        }

        ans //returning answer vector.
    }
}
