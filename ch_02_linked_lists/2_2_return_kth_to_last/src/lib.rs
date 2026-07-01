// Return Kth to Last: Implement an algorithm to find the kth to last element of a singly linked list.

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

/// Solução iterativa com dois ponteiros
/// Tempo: O(N) | Espaço: O(1)
pub fn kth_to_last_iterative(head: &Option<Box<ListNode>>, k: usize) -> Option<&ListNode> {
    unimplemented!()
}

/// Solução recursiva
/// Tempo: O(N) | Espaço: O(N) devido à pilha de chamadas
pub fn kth_to_last_recursive(head: &Option<Box<ListNode>>, k: usize) -> Option<&ListNode> {
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
    fn test_kth_to_last_iterative() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        // k=1 means the last element, k=2 means 2nd to last (4)
        assert_eq!(kth_to_last_iterative(&list, 2).unwrap().val, 4);
    }

    #[test]
    fn test_kth_to_last_recursive() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(kth_to_last_recursive(&list, 2).unwrap().val, 4);
    }
}
