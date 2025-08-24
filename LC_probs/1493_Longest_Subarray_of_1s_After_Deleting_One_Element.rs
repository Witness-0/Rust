impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut starting_point: Vec<i32> = vec![0; n];
        let mut first_pos = 0usize;
        let mut first_pos_occ = false;

        for i in 0..n {
            if nums[i] == 1 {
                if !first_pos_occ {
                    first_pos = i;
                    first_pos_occ = true;
                }
                starting_point[first_pos] += 1;
            } else {
                first_pos_occ = false;
            }
        }

        let mut maxm = 0;

        let mut counter = 0usize;
        while counter < n {
            let val = starting_point[counter];
            if val > 0 {
                maxm = maxm.max(val);

                let next_idx = counter + val as usize + 1;
                if next_idx < n {
                    let new_val = starting_point[next_idx];
                    maxm = maxm.max(val + new_val);
                }

                counter += val as usize + 1;
            } else {
                counter += 1;
            }
        }

        if maxm == n as i32 {
            maxm - 1
        } else {
            maxm
        }
    }
}
/*

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut zero_count = 0;
        let mut max_len = 0;

        for right in 0..n {
            if nums[right] == 0 {
                zero_count += 1;
            }

            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            max_len = max_len.max(right - left);
        }

        max_len as i32
    }
}


*/
