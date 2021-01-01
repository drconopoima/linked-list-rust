type NextNode<T> = Option<Box<ListNode<T>>>;

#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    next: NextNode<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    // Number of non-empty nodes in list
    // @field length
    // @type {usize}
    pub length: usize,
    // Pointer to first node in the list.
    // @field head
    // @type {Option<Box<ListNode<T>>>}
    head: NextNode<T>,
}

impl<T> LinkedList<T> {
    // Creates a new empty instance of LinkedList
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }
    // Creates a new instance of LinkedList from an existing ListNode
    // @param {node: Option<ListNode<T>>} initial node to build list.
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
    // Inserts some value at the start of the list.
    // @param {value: <T>} the data to add to the list.
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }
    // Removes the first value at the start of the list.
    // @returns {Option<T>} the value deleted from the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.value
        })
    }
    // Show first value at the start of the list.
    // @returns {Option<&T>} reference to list's head value
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }
    // Return mut reference to first value at the start of the list.
    // @returns {Option<&T>} mut reference to list's head value
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
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

pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // Element 0 of IntoIter Tuple Struct is LinkedList<T>
        self.0.pop()
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
    #[test]
    fn peek() {
        let mut list: LinkedList<usize> = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.peek_mut().map(|value| {
            *value = 0
        });
        assert_eq!(list.peek(), Some(&0));
        assert_eq!(list.pop(), Some(0));
    }
    #[test]
    fn into_iter() {
        let mut list: LinkedList<f32> = LinkedList::new();
        list.push(1.0);
        list.push(2.0);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), None);
    }
}
