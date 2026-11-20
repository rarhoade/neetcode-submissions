struct MinStack {
    stack: Vec<i32>,
    min_element_stack: Vec<i32>,
}

impl MinStack {
       pub fn new() -> Self {
        let mut min_stack = Vec::new();
        min_stack.push(i32::MAX);
        MinStack {
            stack: Vec::new(),
            min_element_stack: min_stack,
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min_element_stack[self.min_element_stack.len() - 1]
    }

    pub fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    pub fn push(&mut self, val: i32) {
        if val < self.get_min() {
            self.min_element_stack.push(val.clone());
        } else {
            self.min_element_stack.push(self.get_min());
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        self.min_element_stack.pop();
        self.stack.pop();
    } 
}
