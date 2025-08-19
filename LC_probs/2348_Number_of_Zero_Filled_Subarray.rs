/*
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count : i64 = 0;
        for i in 0..n{
            if nums[i] == 0 {
                count +=1;
                let mut j = i;
                let mut sub_count : i64 = 0;
                while j > 0 && nums[j-1] == 0 {
                    sub_count += 1;
                    j -=1;
                }

                count += sub_count; 
            }
        }

        count
    }
}

1 = 1
2 = 3
3 = 6
4 = 10
5 = 15
*/



impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count : i64 = 0;
        let mut sub_count : i64 = 0;

        for i in 0..n{
            if nums[i] == 0 {
                sub_count +=1;
                count += sub_count;
            } else {
                sub_count = 0;
            }
        }

        count
    }
}
