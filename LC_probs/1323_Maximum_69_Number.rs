impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        // Convert number to string for easy digit manipulation
        let mut chars: Vec<char> = num.to_string().chars().collect();

        // Find the first '6' and change it to '9'
        for c in chars.iter_mut() {
            if *c == '6' {
                *c = '9';
                break; // only change the first one
            }
        }

        // Convert back to integer
        chars.into_iter().collect::<String>().parse::<i32>().unwrap()
    }
}
