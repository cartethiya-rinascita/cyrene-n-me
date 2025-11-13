#[derive(Debug)]
pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

impl LinkedList<i32> {
    pub fn new() -> LinkedList<i32> {
        LinkedList {
            val: None,
            next: None,
        }
    }

    pub fn push_left(self, v: i32) -> LinkedList<i32> {
        let node = LinkedList::<i32> {
            val: Some(v),
            next: Some(Box::new(self)),
        };
        node
    }

    pub fn get_val(&self) -> Option<i32> {
        self.val
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self;

        if current.val.is_some() {
            count += 1;
        }

        while let Some(ref next_node) = current.next {
            if next_node.val.is_some() {
                count += 1;
            }

            current = next_node;
        }
        count
    }
}
