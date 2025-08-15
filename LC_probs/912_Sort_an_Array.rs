impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        merge_sort(&mut nums);
        nums
    }
}

fn merge_sort(nums: &mut [i32]) {
    // Base case: already sorted if length <= 1
    if nums.len() <= 1 {
        return;
    }

    // Store length before splitting (avoids borrowing issues later)
    let len = nums.len();
    let mid = len / 2;

    // Split nums into two mutable halves
    let (left, right) = nums.split_at_mut(mid);

    // Sort each half recursively
    merge_sort(left);
    merge_sort(right);

    // Create a temporary vector with enough capacity
    // Use stored `len` instead of calling `nums.len()` here
    let mut merged = Vec::with_capacity(len);

    let mut i = 0;
    let mut j = 0;

    // Merge elements from left and right in sorted order
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    // Append remaining elements from left
    if i < left.len() {
        merged.extend_from_slice(&left[i..]);
    }

    // Append remaining elements from right
    if j < right.len() {
        merged.extend_from_slice(&right[j..]);
    }

    // Copy merged sorted values back into nums
    nums.copy_from_slice(&merged);
}
