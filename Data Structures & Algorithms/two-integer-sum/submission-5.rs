use std::collections::HashMap;

impl Solution {
    /* 
    {
        4: 0, 
        3: 1,
        2: 2,
        1: 3
    }
    */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_targets: HashMap<i32, i32> = HashMap::new(); 
        for (i, num) in nums.iter().enumerate() {
            let i = i as i32;
            let target_val: i32 = target - num;
            index_targets.insert(target_val, i);
        }
        for (i, num) in nums.iter().enumerate() {
            let i = i as i32;
            if index_targets.contains_key(num) && index_targets[num] != i{ 
                return vec![i, index_targets[num]]
            }
        }
        return vec![]
    }
}
