use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

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
    pub fn push_back(&mut self, value: T) {
        let new_tail = ListNode::new(value);
        match self.tail.take() {
            None => {
                // empty-list, need to set the head
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
                self.length += 1;
            }
            Some(old_tail) => {
                // non-empty list, need to connect the old_tail
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
                self.length += 1;
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                    self.length -= 1;
                }
                None => {
                    self.head.take();
                    self.length -= 1;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
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

        // Populate list from front
        list.push_front(1);
        list.push_front(2);
        // Populate list from back
        list.push_back(3);
        list.push_back(4);
        assert_eq!(list.length, 4);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.length, 3);
        // Check normal removal
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        // Check normal removal
        assert_eq!(list.pop_front(), Some(4));
        // Check exhaustion
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.length, 0);
    }
    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&*list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&*list.peek_back_mut().unwrap(), &mut 1);
    }
    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_front(1.0);
        list.push_front(2.0);
        list.push_back(3.0);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next_back(), Some(3.0));
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
