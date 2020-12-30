type NextNode<T>=Box<ListNode<T>>;

#[derive(Debug)]
enum Link<T> {
    Empty,
    More(NextNode<T>),
}

#[derive(Debug)]
pub struct ListNode<T> {
    value: T,
    next: Link<T>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub length: u64,
    head: Link<T>,
    tail: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { 
            head: Link::Empty,
            tail: Link::Empty,
            length: 0
        }
    }
    fn from(node: Link<T>) -> Self {
        match node {
            Link::Empty => {
                LinkedList { 
                    head: Link::Empty,
                    tail: Link::Empty,
                    length: 0
                }
            }
            _ => {
                LinkedList { 
                    head: node,
                    tail: Link::Empty,
                    length: 1
                }
            },
        }
    }
    pub fn push(&mut self, _value: T) {
        unimplemented!();
    }
    pub fn pop(&mut self) -> Option<T> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    unimplemented!();
}

fn main() {
    let link=Link::More(Box::new(ListNode{ value: 1, next: Link::Empty}));
    let linked_list=LinkedList::from(link);
    let empty_list: LinkedList<u32>=LinkedList::from(Link::Empty);
    println!("Populated LinkedList instance: {:#?}", linked_list);
    println!("Empty LinkedList instance: {:#?}", empty_list);
}
