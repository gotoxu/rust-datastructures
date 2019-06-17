#[cfg(test)]
mod tests {
    use crate::queue::PriorityQueue;

    #[test]
    fn priority_queue_test() {
        let mut queue = PriorityQueue::new();
        queue.push(150);
        queue.push(80);
        queue.push(40);
        queue.push(30);
        queue.push(10);
        queue.push(70);
        queue.push(110);
        queue.push(100);
        queue.push(20);
        queue.push(90);
        queue.push(60);
        queue.push(50);
        queue.push(120);
        queue.push(140);
        queue.push(130);

        let min = queue.peek();
        assert_eq!(min, Some(&10));
    }
}

struct PriorityQueue<T: Ord + Copy> {
    items: Vec<T>,
}

impl<T: Ord + Copy> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        let length = self.items.len();
        if length <= 1 {
            return;
        }

        let mut index = length - 1;
        let mut parent = (index - 1) / 2;

        while self.items[parent] > item {
            let temp = self.items[parent];
            self.items[parent] = self.items[index];
            self.items[index] = temp;

            index = parent;
            if index < 1 {
                break;
            }
            parent = (index - 1) / 2
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.items.len() > 0 {
            Some(&self.items[0])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let length = self.items.len();
            let temp = self.items[length - 1];

            self.items[length - 1] = self.items[0];
            self.items[0] = temp;

            let result = self.items.pop().unwrap();

            let mut index = 0;
            let mut child_left = 2 * index + 1;
            let mut child_right = 2 * index + 2;

            while self.items.len() > child_left {
                let mut child = child_left;
                if self.items.len() > child_right
                    && self.items[child_right] < self.items[child_left]
                {
                    child = child_right
                }

                if self.items[child] < self.items[index] {
                    let temp = self.items[child];
                    self.items[child] = self.items[index];
                    self.items[index] = temp;

                    index = child;
                    child_left = 2 * index + 1;
                    child_right = 2 * index + 2;
                } else {
                    break;
                }
            }

            Some(result)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
