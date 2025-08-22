impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start : usize = 0 as usize;
        let mut end : usize = height.len() - 1 as usize;
        let mut area = 0;

        while start < end {

            if height[start] >= height[end] {
                if area < height[end] * (end - start) as i32 {
                    area = height[end] * (end - start) as i32;
                }
            } else {
                if area < height[start] * (end - start) as i32 {
                    area = height[start] * (end - start) as i32;
                }
            }

            if height[start] <= height[end] {
                start +=1;
            } else {
                end -=1;
            }
        }

        area
    }
}


/*
    |
    | |
  | | |
| | | |
  
*/
