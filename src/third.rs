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
    pub fn append(&self, value: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(ListNode {
                value,
                next: self.head.clone(),
            })),
            length: self.length + 1,
        }
    }
    pub fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            length: {
                if self.length > 1 {
                    self.length - 1
                } else {
                    0
                }
            },
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;
    #[test]
    fn basics() {
        let list: LinkedList<u32> = LinkedList::new();
        assert_eq!(list.head(), None);
        let list = list.append(1).append(2);
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
