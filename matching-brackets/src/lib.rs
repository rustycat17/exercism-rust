pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for letter in string.chars() {
        match letter {
            '(' | '[' | '{' => stack.push(letter),
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                } else {
                    stack.push(letter);
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    stack.push(letter);
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    stack.push(letter);
                }
            }
            _ => {}
        };
    }
    stack.is_empty()
}