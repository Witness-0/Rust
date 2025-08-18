impl Solution {

    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let mut ans : Vec<Vec<i32>> = Vec::new();
        let n = nums.len();

        if n < 4 {
            return ans;
        }

        nums.sort_unstable();
        let target_64 = target as i64;

        for i in 0..n-3 {

            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let min_si = nums[i] as i64 + nums[i+1] as i64 + nums[i+2] as i64 + nums[i+3] as i64;
            if min_si > target_64 {
                break;
            }

            let max_si = nums[i] as i64 + nums[n-1] as i64 + nums[n-2] as i64 + nums[n-3] as i64;
            if max_si < target_64 {
                continue;
            }

            for j in i+1..n-2 {

                if j > i+1 && nums[j] == nums[j-1] {
                    continue;
                }

                let min_sj = nums[i] as i64 + nums[j] as i64 + nums[j+1] as i64 + nums[j+2] as i64;
                if min_sj > target_64 {
                    break;
                }

                let max_sj = nums[i] as i64 + nums[j] as i64 + nums[n-1] as i64 + nums[n-2] as i64;
                if max_sj < target_64 {
                    continue;
                }

                let (mut l, mut r) = (j+1, n-1);

                while l < r {

                    let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                    if sum == target_64 {
                        ans.push(vec![nums[i],nums[j],nums[l],nums[r]]);

                        let temp_l = nums[l];
                        while l < r && nums[l] == temp_l {
                            l +=1;
                        }

                        let temp_r = nums[r];
                        while l < r && nums[r] == temp_r {
                            r -=1;
                        }

                    } else if sum > target_64 {
                            r -=1;
                    } else{
                        l +=1;
                    }
                }
            }
        }
        ans
    }
}
