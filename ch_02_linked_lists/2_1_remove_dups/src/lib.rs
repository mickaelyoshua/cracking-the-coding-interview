// Remove Dups: Write code to remove duplicates from an unsorted linked list.
// 
// FOLLOW UP
// How would you solve this problem if a temporary buffer is not allowed?

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

/// Solução com buffer (ex: HashSet)
/// Tempo: O(N) | Espaço: O(N)
pub fn remove_dups_with_buffer(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unimplemented!()
}

/// Solução sem buffer temporário
/// Tempo: O(N^2) | Espaço: O(1)
pub fn remove_dups_no_buffer(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a list from a vec
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
    fn test_remove_dups_with_buffer() {
        let list = to_list(vec![1, 2, 3, 2, 4, 1]);
        let expected = to_list(vec![1, 2, 3, 4]);
        assert_eq!(remove_dups_with_buffer(list), expected);
    }

    #[test]
    fn test_remove_dups_no_buffer() {
        let list = to_list(vec![1, 2, 3, 2, 4, 1]);
        let expected = to_list(vec![1, 2, 3, 4]);
        assert_eq!(remove_dups_no_buffer(list), expected);
    }
}
