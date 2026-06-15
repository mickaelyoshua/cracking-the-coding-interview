pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub next: Link<T>,
}
impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    pub head: Link<T>,
}

impl<T> LinkedList<T> {
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
