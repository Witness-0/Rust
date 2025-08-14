impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // Edge case: if the input vector is empty, return an empty string immediately.
        if strs.is_empty() {
            return String::new();
        }

        // Step 1: Find the lexicographically smallest and largest strings.
        // Why? The common prefix of the entire array is guaranteed to be
        // the common prefix of these two extremes.
        let mut min_str = &strs[0]; // reference to the smallest so far
        let mut max_str = &strs[0]; // reference to the largest so far

        // Iterate through all strings in the vector
        for s in &strs {
            if s < min_str {
                min_str = s; // found a new smallest string
            }
            if s > max_str {
                max_str = s; // found a new largest string
            }
        }

        // Step 2: Find how many characters match from the start (prefix length)

        let prefix_len = min_str
            .chars()                       // iterate over chars of smallest string
            .zip(max_str.chars())           // pair with chars from largest string
            .take_while(|(a, b)| a == b)    // keep going while chars are equal
            .count();                       // count how many matched chars

        // Step 3: Extract the prefix of that length from `min_str`

        // `chars()` again creates a fresh iterator over min_str's characters.
        // `.take(prefix_len)` takes only the matching prefix_len characters.
        // `.collect()` gathers those chars into a new String (UTF-8 encoded automatically).
        min_str
            .chars()
            .take(prefix_len)
            .collect()
    }
}
