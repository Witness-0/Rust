/* use std::collections::HashSet; */

impl Solution {

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();                      // 1) sort in-place
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let n = nums.len();

    for i in 0..n {                   // 2) fix one element at index i
        if i > 0 && nums[i] == nums[i - 1] {
            continue;                 // 3) skip same fixed value to avoid duplicate triplets
        }

        let (mut l, mut r) = (i + 1, n - 1); // 4) two pointers after i
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            if sum == 0 {
                // 5) found a triplet
                ans.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;
                r -= 1;

                // 6) skip duplicate second/third values
                while l < r && nums[l] == nums[l - 1] { l += 1; }
                while l < r && nums[r] == nums[r + 1] { r -= 1; }

            } else if sum < 0 {
                // 7) sum too small → increase it by moving l right
                l += 1;

            } else {
                // 8) sum too large → decrease it by moving r left
                r -= 1;
            }
            }
        }

        ans
    }
}

   /* pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let size = nums.len();
        let mut ans: HashSet<Vec<i32>> = HashSet::new(); // use set to avoid duplicates

        for i in 0..size {
            let fixed = nums[i];
            let target = -fixed;
            let mut map = std::collections::HashMap::new();

            for &num in nums.iter().skip(i + 1) {
                let comp = target - num;

                if let Some(_) = map.get(&comp) {
                    let mut triplet = vec![fixed, num, comp];
                    triplet.sort(); // sort each triplet to avoid ordering differences
                    ans.insert(triplet);
                }

                map.insert(num, true);
            }
        }

        ans.into_iter().collect()
    }
}
*/
