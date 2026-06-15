pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: Link<T>,
}
impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

#[derive(Default, Debug)]
pub struct LinkedList<T> {
    pub head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, val: T) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            val,
            next: old_head,
        });

        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        let mut current_link = &mut self.head;
        while let Some(node) = current_link {
            current_link = &mut node.next;
        }

        let new_node = Box::new(Node::new(val));
        *current_link = Some(new_node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.head.as_ref()?;

        let mut current_link = &mut self.head;
        while current_link.as_ref().unwrap().next.as_ref().is_some() {
            current_link = &mut current_link.as_mut().unwrap().next;
        }

        let node = current_link.take().unwrap();
        Some(node.val)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Some(mut node) = current_link {
            current_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn pop_empty_return_none() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn push_and_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn push_and_pop_back() {
        let mut list = LinkedList::new();

        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
    }

    #[test]
    fn drop_without_stack_overflow() {
        let mut list = LinkedList::new();

        for i in 0..100_000 {
            list.push_front(i);
        }
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_push_pop_front(items in prop::collection::vec(any::<i32>(), 0..100)) {
            let mut list = LinkedList::new();

            for &item in items.iter() {
                list.push_front(item);
            }
            for &item in items.iter().rev() {
                assert_eq!(list.pop_front(), Some(item));
            }

            assert_eq!(list.pop_front(), None);
        }

        #[test]
        fn prop_push_pop_back(items in prop::collection::vec(any::<i32>(), 0..100)) {
            let mut list = LinkedList::new();

            for &item in items.iter() {
                list.push_back(item);
            }
            for &item in items.iter().rev() {
                assert_eq!(list.pop_back(), Some(item));
            }

            assert_eq!(list.pop_back(), None);

        }
    }
}
