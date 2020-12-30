use std::mem;

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
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { 
            head: Link::Empty,
            length: 0
        }
    }
    fn from(node: Link<T>) -> Self {
        match node {
            Link::Empty => {
                LinkedList { 
                    head: Link::Empty,
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
    pub fn push(&mut self, value: T) {
        let new_node = Box::new( ListNode {
            value,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
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
    let link=Link::More(Box::new(ListNode{ value: "1", next: Link::Empty}));
    let linked_list=LinkedList::from(link);
    let mut empty_list: LinkedList<u32>=LinkedList::from(Link::Empty);
    println!("Populated LinkedList instance: {:#?}", linked_list);
    println!("Empty LinkedList instance: {:#?}", empty_list);
    empty_list.push(32);
    println!("Pushed into LinkedList instance: {:#?}", empty_list);
}
