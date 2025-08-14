use std::collections::HashMap; //This imports the HashMap data structure from Rust's standard library.

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut seen: HashMap<i32, usize> = HashMap::new(); // Creates a mutable hash map that maps values (i32) to their indices (usize).

        // iterate over nums with indices
        // Example: if nums = [2, 7, 11, 15], we get:
        //   i=0, val=2
        //   i=1, val=7  
        //   i=2, val=11
        //   i=3, val=15

        for (i, &val) in nums.iter().enumerate() {
            let need = target - val; // the complement we are looking for

            // check if we have already seen the needed value before
            if let Some(&j) = seen.get(&need) {
                // found indices j and i such that nums[j] + nums[i] == target
                // LeetCode expects Vec<i32>; indices are usize, so cast them
                return vec![j as i32, i as i32];
            }

            // otherwise record current value->index for future matches
            // note: i32 implements Copy, so we store val by value (cheap)
            seen.insert(val, i);
        }

        // per problem statement there's exactly one solution; unreachable in valid tests
        vec![]
    }
}


/* 
c++ equivalent for my understanding :
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> seen;
        for (int i = 0; i < nums.size(); i++) {
            int need = target - nums[i];
            if (seen.find(need) != seen.end()) {
                return { seen[need], i };
            }
            seen[nums[i]] = i;
        }
        return {};
    }
};
*/
