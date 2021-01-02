use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

struct ListNode<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn push_front(&mut self, value: T) {
        let new_head = ListNode::new(value);
        match self.head.take() {
            None => {
                // empty-list, need to set the tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
                self.length += 1;
            }
            Some(old_head) => {
                // non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
                self.length += 1;
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    self.length -= 1;
                }
                None => {
                    self.tail.take();
                    self.length -= 1;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;
    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        // Check normal removal
        assert_eq!(list.pop_front(), Some(4));
        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
