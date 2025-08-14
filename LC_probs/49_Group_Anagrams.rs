use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 1 Create a hash map.
        // Key: the sorted version of a string (anagram signature)
        // Value: vector of strings that match this signature (all anagrams)
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        // 2 Loop through each string in the input
        for s in strs {
            // Convert the string to a vector of characters
            // Rust strings are UTF-8, so we need a Vec<char> to sort characters
            let mut chars: Vec<char> = s.chars().collect();

            // Sort the characters alphabetically
            // e.g., "eat" -> ['e','a','t'] -> ['a','e','t']
            chars.sort_unstable(); // unstable sort is faster and good enough here

            // Convert the sorted vector of chars back into a string
            // This will be our key in the hash map
            let sorted_s: String = chars.into_iter().collect();

            // Use the hash map entry API:
            // - If the key exists, get the existing vector
            // - If the key does NOT exist, insert a new empty vector
            // Then push the original string into that vector
            map.entry(sorted_s)
                .or_insert_with(Vec::new) // create new vector if key not found
                .push(s); // move the original string into the vector
        }

        // 3 Collect all the vectors from the hash map into a single Vec<Vec<String>>
        // - This drops the hash map and moves out the values
        // - Each vector contains all anagrams for its key
        map.into_values().collect()
    }
}
