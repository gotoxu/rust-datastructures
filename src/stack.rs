#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn it_works() {
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(15);
        stack.push(22);
        stack.push(19);
        stack.push(11);
        stack.push(89);

        assert_eq!(false, stack.is_empty());
        assert_eq!(89, stack.peek().unwrap().clone());
        assert_eq!(89, stack.pop().unwrap().clone());

        assert_eq!(11, stack.peek().unwrap().clone());
    }
}

pub struct Stack<T> {
    entries: Vec<T>,
    top: i32,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            entries: vec![],
            top: -1,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.top == -1;
    }

    pub fn size(&self) -> usize {
        return self.entries.len();
    }

    pub fn push(&mut self, value: T) {
        self.entries.push(value);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top < 0 {
            None
        } else {
            self.top -= 1;
            self.entries.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top < 0 {
            None
        } else {
            self.entries.get(self.top as usize)
        }
    }
}
