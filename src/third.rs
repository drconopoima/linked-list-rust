use std::rc::Rc;

type Link<T> = Option<Rc<ListNode<T>>>;

#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Link<T>,
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
    head: Link<T>,
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

pub struct Iter<'a, T> {
    next: Option<&'a ListNode<T>>,
}
impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
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
    #[test]
    fn iter() {
        let list: LinkedList<Option<Vec<i32>>> =
            LinkedList::new().append(Some(Vec::new())).append(None);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(Vec::new())));
        assert_eq!(iter.next(), None);
    }
}
