// A simple fixed-size HashMap implementation for integer keys in the range 0..=1,000,000
struct MyHashMap {
    // The vector index represents the key, and the stored i32 is the value.
    // A value of -1 means "no mapping for this key".
    map: Vec<i32>,
}

impl MyHashMap {
    // Creates a new MyHashMap instance
    fn new() -> Self {
        Self {
            // Allocate a vector of size 1,000,001 (for keys 0 to 1,000,000)
            // Initialize all positions to -1 to indicate "not present"
            map: vec![-1; 1_000_001],
        }
    }
    
    // Inserts a key-value pair into the hash map
    // If the key already exists, its value is updated
    fn put(&mut self, key: i32, value: i32) {
        self.map[key as usize] = value; // Direct index access for O(1) operation
    }
    
    // Retrieves the value associated with the given key
    // Returns -1 if the key is not present
    fn get(&self, key: i32) -> i32 {
        self.map[key as usize] // Directly returns the stored value
    }
    
    // Removes the mapping for the given key (if it exists)
    fn remove(&mut self, key: i32) {
        self.map[key as usize] = -1; // Reset to sentinel value
    }
}
