fn main() {}

pub fn is_valid(s: String) -> bool {
    use std::collections::VecDeque;
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut rst = true;
    for i in s.chars() {
        match i {
            ')' => {
                if stack.back().is_none() || stack.back().unwrap() != &'(' {
                    rst = false;
                    break;
                } else {
                    stack.pop_back();
                }
            }
            ']' => {
                if stack.back().is_none() || stack.back().unwrap() != &'[' {
                    rst = false;
                    break;
                } else {
                    stack.pop_back();
                }
            }
            '}' => {
                if stack.back().is_none() || stack.back().unwrap() != &'{' {
                    rst = false;
                    break;
                } else {
                    stack.pop_back();
                }
            }
            _ => stack.push_back(i),
        }
    }
    if !stack.is_empty() {
        rst = false;
    }
    rst
}
