impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }  // by default not an anagram since theres an extra character in either of string making them different.

        let mut count = vec![0;26]; // we declared a mutable vector [since we are gonna change the elements in it according to the count], size = 26, all elements = 0.

        for ch in s.chars() {
            count[(ch as u8 - b'a') as usize] +=1; // by using ch as u8, we convert it into its ascii byte value, similarly adding b before 'a' does the same. We then cast to usize to use it as an index.
        }

        for ch in t.chars() {
            count[(ch as u8 - b'a') as usize] -=1; // subtracting the same way — cancels out matching characters.
        }

        // now if we had same characters in both the string they would be plused and minused the exactly same time, making the vector full of zeroes. so we just check for non zero occurances.

        count.iter().all(|&c| c==0) // we iterate over each element and all(|&c| == 0) checks if each element is zero.
    }

    /*
    ch is a char (Unicode scalar value, 4 bytes).
    ch as u8 converts the character to its ASCII byte value (this works for 'a'..'z' but not for all Unicode characters).
    b'a' is the byte literal for lowercase 'a' (value 97 in ASCII).
    (ch as u8 - b'a') is the position of the letter in the alphabet (e.g., 'a' → 0, 'b' → 1, etc.).
    The result of (ch as u8 - b'a') is still u8.

    Vectors in Rust must be indexed by a usize, not a u8.
    So u8 → usize casting is required.
    */


}
