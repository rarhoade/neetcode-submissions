impl Solution {
    pub fn is_valid(s: String) -> bool {
let mut stack = VecDeque::new();
    for c in s.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push_back(c);
        } else {
            match stack.pop_back() {
                Some(val) => {
                    if c == '}' {
                        if val != '{' {
                            return false;
                        }
                    }
                    if c == ']' {
                        if val != '[' {
                            return false;
                        }
                    }
                    if c == ')' {
                        if val != '(' {
                            return false;
                        }
                    }
                }
                None => { return false;} ,
            }
        }
    }
    if !stack.is_empty() {
        return false
    }
    return true;
    }
}
