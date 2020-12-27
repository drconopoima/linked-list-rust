#[derive(Debug)]
struct LinkedNode<T> {
    value: T,
    next: Option<Box<LinkedNode<T>>>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub length: u64,
    head: Option<Box<LinkedNode<T>>>,
    // tail: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    fn new(value: T, next: Option<Box<LinkedNode<T>>>) -> LinkedNode<T> {
        LinkedNode {
            value,
            next
        }
    }
}

impl<T> LinkedList<T> {
    fn new(node: Option<Box<LinkedNode<T>>>) -> LinkedList<T> {
        match node {
            None => {
                LinkedList { 
                    head: None,
                    length: 0
                }
            }
            _ => {
                LinkedList { 
                    head: node,
                    length: 1
                }
            },
        }
    }
}

fn main() {
    let linked_node=LinkedNode::new(1, Option::None);
    let linked_list=LinkedList::new(Option::from(Box::from(linked_node)));
    let empty_list: LinkedList<u32>=LinkedList::new(None);
    println!("Populated LinkedList instance: {:#?}", linked_list);
    println!("Empty LinkedList instance: {:#?}", empty_list);
}
