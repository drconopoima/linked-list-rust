type NextNode<T>=Option<Box<ListNode<T>>>;

#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    next: NextNode<T>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub length: u64,
    head: NextNode<T>,
    tail: NextNode<T>,
}

impl<T> ListNode<T> {
    fn new(value: T, next: NextNode<T>) -> ListNode<T> {
        ListNode {
            value,
            next
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { 
            head: None,
            tail: None,
            length: 0
        }
    }
    pub fn from(node: NextNode<T>) -> Self {
        match node {
            None => {
                LinkedList { 
                    head: None,
                    tail: None,
                    length: 0
                }
            }
            _ => {
                LinkedList { 
                    head: node,
                    tail: None,
                    length: 1
                }
            },
        }
    }
}

fn main() {
    let linked_node=ListNode::new(1, None);
    let linked_list=LinkedList::from(Some(Box::from(linked_node)));
    let empty_list: LinkedList<u32>=LinkedList::from(None);
    println!("Populated LinkedList instance: {:#?}", linked_list);
    println!("Empty LinkedList instance: {:#?}", empty_list);
}
