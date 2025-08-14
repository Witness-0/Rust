impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let n = num.len();

        // Early exit: if length < 3, it's impossible to have 3 consecutive same digits
        if n < 3 {
            return "".to_string();
        }

        let mut ans: i32 = -1; // store largest good integer found (start with "none found")

        // Slide over each window of 3 consecutive characters
        for i in 0..=n - 3 {
            let sub_str = &num[i..i + 3];

            // Convert just this 3-digit substring into a number
            let sub_num: i32 = sub_str.parse().unwrap();

            // Check if all digits are the same: multiple of 111 (000, 111, 222, ..., 999)
            if sub_num % 111 == 0 {
                if sub_num > ans {
                    ans = sub_num;
                }
            }
        }

        // Return the result
        if ans == -1 {
            "".to_string()
        } else {
            format!("{:03}", ans) // keep leading zeros like "000"
        }
    }
}
