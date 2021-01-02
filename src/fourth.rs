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
