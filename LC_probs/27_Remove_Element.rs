impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Keep only elements that are NOT equal to val
        let mut write_index = 0;

        for read_index in 0..nums.len() {
            // think of our aarya as [ VALID ZONE | UNKNOWN ZONE ]
            // If nums[read_index] != val → copy it into VALID ZONE, expand the zone by moving write_index forward
            //else do nothing, explore unknown zone.


            if nums[read_index] != val {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        write_index as i32 // This is the "new length" , think of it as a valid zone
    }
}
