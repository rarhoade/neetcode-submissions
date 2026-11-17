impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut results: Vec<Vec<i32>> = vec![];
    for (index, value) in sorted_nums.iter().enumerate() {
        if index > 0 && *value == sorted_nums[index - 1] {
            continue;
        }
        let mut j = index + 1;
        let mut k = sorted_nums.len() - 1;
        while j < k {
            let three_sum_target = *value + sorted_nums[j] + sorted_nums[k];
            if three_sum_target < 0 {
                j = j + 1;
            } else if three_sum_target > 0 {
                k = k - 1;
            } else {
                results.push(vec![*value, sorted_nums[j], sorted_nums[k]]);
                j = j + 1;
                while j < sorted_nums.len() && sorted_nums[j] == sorted_nums[j - 1] {
                    j = j + 1;
                }
            }
        }
    }
    return results; 
    }
}
