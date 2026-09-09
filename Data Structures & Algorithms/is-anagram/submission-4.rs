impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_store: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let val = s_store.get(&c).clone();
            match val {
                Some(value) => s_store.insert(c, value + 1) ,
                None => s_store.insert(c, 1)
            };
        }

        for c in t.chars() {
            let val = s_store.get(&c).clone();
            match val {
                Some(value) => {
                    let new_val = value - 1;
                    if new_val < 0 {
                        return false;
                    }
                    s_store.insert(c, new_val);
                    
                },
                None => return false
            }
        }
        for key in s_store.keys() {
            let val = s_store.get(key);
            match val {
                Some(value) => {
                    if *value != 0 {
                        return false
                    }
                },
                None => return false
            }
        }
        return true;
    }
}
