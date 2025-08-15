// A simple hash set implementation for integer keys in the range 0..=1,000,000
struct MyHashSet {
    // `data[i]` is `true` if key `i` is present, else `false`
    data: Vec<bool>,
}

impl MyHashSet {
    // Creates a new MyHashSet instance
    fn new() -> Self {
        MyHashSet {
            // Pre-allocate a vector of size 1_000_001 (for keys 0 to 1,000,000)
            // All values are initially `false` meaning "key not present"
            data: vec![false; 1_000_001],
        }
    }

    // Adds a key to the set
    fn add(&mut self, key: i32) {
        // Mark the position corresponding to `key` as `true`
        self.data[key as usize] = true;
    }

    // Removes a key from the set
    fn remove(&mut self, key: i32) {
        // Mark the position corresponding to `key` as `false`
        self.data[key as usize] = false;
    }

    // Checks whether the set contains the given key
    fn contains(&self, key: i32) -> bool {
        // Directly return the boolean at index `key`
        self.data[key as usize]
    }
}
