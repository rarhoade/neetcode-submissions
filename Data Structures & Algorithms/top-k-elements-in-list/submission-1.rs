impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count_hash: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            *count_hash.entry(i).or_insert(0) += 1;
        }

        let mut prio_queue = BinaryHeap::new();
        for (val, count) in count_hash {
            prio_queue.push((count, val));
        }

        let mut return_vec: Vec<i32> = vec![];
        for _i in 0..k {
            let tuple = prio_queue.pop();  
            match tuple {
                Some((prio, val)) => return_vec.push(val),
                _ => {}
            }
        }
        return return_vec;
    }
}
