// Partition: Write code to partition a linked list around a value x, such that all nodes less than x come
// before all nodes greater than or equal to x. If x is contained within the list, the values of x only need
// to be after the elements less than x (see below). The partition element x can appear anywhere in the
// "right partition"; it does not need to appear between the left and right partitions.
// EXAMPLE
// Input: 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1 [partition=5]
// Output: 3 -> 1 -> 2 -> 10 -> 5 -> 5 -> 8

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

/// Particiona a lista preservando (ou não) a ordem relativa
/// Tempo: O(N) | Espaço: O(1)
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
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
    fn test_partition() {
        let list = to_list(vec![3, 5, 8, 5, 10, 2, 1]);
        let mut result = partition(list, 5);
        
        // Verifica se todos menores que 5 vem antes dos maiores ou iguais a 5
        let mut seen_greater_or_equal = false;
        while let Some(node) = result {
            if node.val >= 5 {
                seen_greater_or_equal = true;
            } else {
                assert!(!seen_greater_or_equal, "Found {} after a value >= 5", node.val);
            }
            result = node.next;
        }
    }
}
