impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
if numbers.len() == 0 {
        return vec![];
    }
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        let sum = numbers[i] + numbers[j] as i32;
        if sum == target {
            return vec![i as i32 + 1, j as i32 + 1];
        }
        if sum > target {
            j = j - 1;
        } else {
            i = i + 1;
        }
    }
    return vec![];
    }
}
