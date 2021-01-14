pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars() {
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);
        } else {
            let valid = match c {
                ')' => stack.pop() == Some('('),
                '}' => stack.pop() == Some('{'),
                ']' => stack.pop() == Some('['),
                _ => true,
            };
            if !valid {
                return false;
            }
        }
    }
    stack.len() == 0
}
