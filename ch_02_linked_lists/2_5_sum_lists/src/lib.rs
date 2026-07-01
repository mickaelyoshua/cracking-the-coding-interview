// Sum Lists: You have two numbers represented by a linked list, where each node contains a single
// digit. The digits are stored in reverse order, such that the 1 's digit is at the head of the list. Write a
// function that adds the two numbers and returns the sum as a linked list.
// EXAMPLE
// Input: (7-> 1 -> 6) + (5 -> 9 -> 2). That is, 617 + 295.
// Output: 2 -> 1 -> 9. That is, 912.
// FOLLOW UP
// Suppose the digits are stored in forward order. Repeat the above problem.
// EXAMPLE
// Input: (6 -> 1 -> 7) + (2 -> 9 -> 5). That is, 617 + 295.
// Output: 9 -> 1 -> 2. That is, 912.

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

/// Soma duas listas com dígitos em ordem reversa
/// Tempo: O(N) | Espaço: O(N) para criar a nova lista
pub fn sum_lists_reverse_order(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unimplemented!()
}

/// Soma duas listas com dígitos em ordem normal
/// Tempo: O(N) | Espaço: O(N)
pub fn sum_lists_forward_order(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    fn test_sum_lists_reverse_order() {
        let l1 = to_list(vec![7, 1, 6]);
        let l2 = to_list(vec![5, 9, 2]);
        let expected = to_list(vec![2, 1, 9]);
        assert_eq!(sum_lists_reverse_order(l1, l2), expected);
    }

    #[test]
    fn test_sum_lists_forward_order() {
        let l1 = to_list(vec![6, 1, 7]);
        let l2 = to_list(vec![2, 9, 5]);
        let expected = to_list(vec![9, 1, 2]);
        assert_eq!(sum_lists_forward_order(l1, l2), expected);
    }
}
