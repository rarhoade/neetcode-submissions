impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_store: HashMap<u8, i32> = HashMap::new(); 
        let mut t_store: HashMap<u8, i32> = HashMap::new(); 
        for (a, b) in s.bytes().zip(t.bytes()) {
            *s_store.entry(a).or_insert(0) += 1;
            *t_store.entry(b).or_insert(0) += 1;
        } 

        return s_store == t_store;
    }
}
