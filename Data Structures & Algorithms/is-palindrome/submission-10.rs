impl Solution {
    pub fn is_palindrome(s: String) -> bool {
let ignored_s: String = s
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphanumeric())
        .collect();

    if ignored_s.len() <= 0 {
        return true;
    }

    let mut i = 0;
    let mut j = ignored_s.len() - 1;
    while i < j {
        if ignored_s.chars().nth(i) != ignored_s.chars().nth(j) {
            return false;
        }
        i = i + 1;
        j = j - 1;
    }
    return true; 
    }
}
