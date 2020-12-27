#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub length: u64,
    head: Option<ListNode<T>>,
    tail: Option<ListNode<T>>,
}

fn main() {
    println!("Hello, world!");
}
