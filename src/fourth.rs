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
}
