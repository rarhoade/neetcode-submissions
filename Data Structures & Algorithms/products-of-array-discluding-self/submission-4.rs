impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut pre_append = Vec::new();
        let mut post_append = vec![0; len];
        let mut pre_app_value = 1;
        for num in nums.clone() {
            pre_append.push(pre_app_value.clone());
            pre_app_value *= num;
        } 

        let mut post_app_value = 1;
        let mut i: isize = (nums.len() - 1).try_into().unwrap();

        while i >= 0 {
            post_append[i as usize] = post_app_value.clone();
            post_app_value *= nums[i as usize];
            i = i - 1;
        }


        let mut ret_vec = Vec::new();
        let mut i = 0;
        while i < len {
            let value = pre_append[i] * post_append[i];
            ret_vec.push(value);
            i = i + 1;
        }
        return ret_vec;
    }
}
