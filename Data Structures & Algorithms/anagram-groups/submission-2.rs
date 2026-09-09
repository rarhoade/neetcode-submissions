impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
       let mut ana_hash: HashMap<String, Vec<String>> = HashMap::new();
        for str_val in strs {
            let mut key_chars: Vec<char> = str_val.chars().collect();
            key_chars.sort_unstable();
            let key_string: String = key_chars.into_iter().collect();
            ana_hash
                .entry(key_string)
                .or_insert_with(Vec::new)
                .push(str_val);
        }
    
        let mut return_vec = vec![];
        for value in ana_hash.values() {
            return_vec.push(value.clone());
        }
        return return_vec; 
    }
}
