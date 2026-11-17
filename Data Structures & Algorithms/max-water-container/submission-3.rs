use std::cmp;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut i = 0;
    let mut j = heights.len() - 1;
    let mut stored_total = 0;
    while i < j {
        let current_water = (j - i) as i32 * cmp::min(heights[i], heights[j]);
        if current_water > stored_total {
            stored_total = current_water;
        }
        if heights[i] < heights[j] {
            i = i + 1;
        } else {
            j = j - 1;
        }
    }
    return stored_total;
    }
}
