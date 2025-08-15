impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // Must be positive
        if n <= 0 {
            return false;
        }

        // Check if n is a power of two (only one bit set)
        if (n & (n - 1)) != 0 {
            return false;
        }

        // Check if the '1' bit is in an even position (power of four property)
        // 0x5555_5555 = binary pattern with 1's at even bit positions = 01010101 01010101 01010101 01010101
        //00000000 00000000 00000000 00000001 = 1
        //00000000 00000000 00000000 00000100 = 4
        //00000000 00000000 00000000 00010000 = 16
        (n & 0x5555_5555) != 0
    }
}


// another way, which works for all such questions
// replace denom with power asked in ques.

/*
impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

*/
