use std::rc::Rc;

type NextNode<T> = Option<Rc<ListNode<T>>>;

#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: NextNode<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    // Number of non-empty nodes in list
    // @field length
    // @type {usize}
    length: usize,
    // Pointer to first node in the list.
    // @field head
    // @type {Option<Box<ListNode<T>>>}
    head: NextNode<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }
}
