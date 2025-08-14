impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool { //introducted mut before nums: (since we are sorting the vector)
        nums.sort(); //sorted the array in nlogn TC.
        for i in 1..nums.len(){ //started from 1 to avoid index out of bounds errors
            if(nums[i-1]==nums[i]){ // if come across dups, we return true
                return true;
            }
        }

        false // else false
    }
}
