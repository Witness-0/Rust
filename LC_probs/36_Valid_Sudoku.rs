use std::cmp;

/// LeetCode-style signature
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // rows[r][d] == true  => digit d (0..=8 meaning '1'..='9') seen in row r
        // cols[c][d] == true  => digit d seen in column c
        // boxes[b][d] == true => digit d seen in 3x3 box b (0..=8)
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue; // empty cell, skip
                }

                // Convert '1'..'9' -> 0..8
                let d = (ch as u8 - b'1') as usize;

                // Which 3×3 box? (top-left box is 0, next is 1, ... rightmost is 2; next row of boxes adds +3, etc.)
                let b = (r / 3) * 3 + (c / 3);

                // If we’ve already seen this digit in the row/col/box, it's invalid.
                if rows[r][d] || cols[c][d] || boxes[b][d] {
                    return false;
                }

                // Mark digit as seen in the current row, column, and box.
                rows[r][d] = true;
                cols[c][d] = true;
                boxes[b][d] = true;
            }
        }

        // No conflicts found
        true
    }
}
