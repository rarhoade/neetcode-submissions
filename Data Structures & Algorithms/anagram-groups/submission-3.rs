impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ana_hash: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for str_val in strs {
            let mut key_chars: Vec<u8> = vec![0; 26];
            for t in str_val.chars() {
                let c = t as u8;
                key_chars[c as usize - 'a' as usize] += 1;
            }
            ana_hash
                .entry(key_chars)
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
