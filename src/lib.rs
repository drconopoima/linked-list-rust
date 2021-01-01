type NextNode<T> = Option<Box<ListNode<T>>>;

#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    next: NextNode<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub length: u64,
    head: NextNode<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }
    pub fn from(node: Option<ListNode<T>>) -> Self {
        match node {
            None => LinkedList {
                head: None,
                length: 0,
            },
            Some(listnode) => LinkedList {
                head: Some(Box::new(listnode)),
                length: 1,
            },
        }
    }
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.value
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None by
            // take() method, so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let node = ListNode {
            value: "1",
            next: None,
        };

        let mut empty_list: LinkedList<&str> = LinkedList::from(None);

        // Test empty list pop behaviour
        assert_eq!(empty_list.pop(), None);
        // Test adding pre-existing node elements using from
        let mut linked_list = LinkedList::from(Some(node));
        let mut mutable_list = empty_list;

        mutable_list.push("1");

        // Popping &str "1" should equal popping &str "1"
        assert_eq!(linked_list.pop(), mutable_list.pop());

        // Create list of other types
        let mut list = LinkedList::new();
        // Populate list
        list.push(1);
        assert_eq!(list.length, 1);
        list.push(2);

        // Check removal
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.length, 1);
        // Push more items to check nothing has corrupted
        list.push(3);
        list.push(4);
        assert_eq!(list.length, 3);
        // Check normal removal
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);
    }
}
