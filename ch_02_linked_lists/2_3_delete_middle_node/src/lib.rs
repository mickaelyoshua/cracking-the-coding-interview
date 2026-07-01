// Delete Middle Node: Implement an algorithm to delete a node in the middle (i.e., any node but
// the first and last node, not necessarily the exact middle) of a singly linked list, given only access to
// that node.
// EXAMPLE
// Input: the node c from the linked list a->b->c->d->e->f
// Result: nothing is returned, but the new linked list looks like a->b->d->e->f

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Deleta o nó do meio "in-place" se não for o último nó
/// Tempo: O(1) | Espaço: O(1)
pub fn delete_middle_node(node: &mut ListNode) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn test_delete_middle_node() {
        let mut list = to_list(vec![1, 2, 3, 4, 5]);
        
        // Vamos extrair a referência mutável para o nó '3' e passá-lo
        // list -> 1 -> 2 -> 3 -> 4 -> 5
        if let Some(ref mut node_1) = list {
            if let Some(ref mut node_2) = node_1.next {
                if let Some(ref mut node_3) = node_2.next {
                    delete_middle_node(node_3);
                }
            }
        }

        let expected = to_list(vec![1, 2, 4, 5]);
        assert_eq!(list, expected);
    }
}
