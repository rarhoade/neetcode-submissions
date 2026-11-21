impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() as i32 - 1;
    
        while i <= j {
            let target_index = (j + i) / 2;
            let current = nums[target_index as usize];
    
            if current == target {
                return target_index;
            }
    
            if current < target {
                i = target_index + 1;
            } else {
                j = target_index - 1;
            }
        }
    
        return -1; 
    }
}
